import {
  parseSpecs,
  processInterfaces,
  processNamespaces,
} from "./parsers/spec-parser.js";
import { DependencyResolver } from "./parsers/dependency-resolver.js";
import { generateEnums } from "./generators/enum-generator.js";
import { generateNamespace } from "./generators/namespace-generator.js";
import { generateInterface } from "./generators/interface-generator.js";
import { generateDictionary } from "./generators/dictionary-generator.js";
import { enums, dictOwner } from "./globals.js";
// import { writeFileSync } from "fs";

/**
 * Main generation orchestrator for emlite-bind
 * Refactored from monolithic 544-line gen.js into modular architecture
 * matching wasmbind's systematic approach
 *
 * @param {Object} specAst - Parsed WebIDL specifications
 */
export function generate(specAst) {
  console.log("Starting emlite-bind generation with modular architecture...");

  // Parse the WebIDL specifications
  const { interfaces, mixins, includeRel, dicts, namespaces } =
    parseSpecs(specAst);

  // Process interfaces (merge partials, includes, etc.)
  const processedInterfaces = processInterfaces(interfaces, mixins, includeRel);

  // Process namespaces (merge partials)
  const processedNamespaces = processNamespaces(namespaces);

  // Create dependency resolver to systematically handle type dependencies
  // This replaces the "not sure how to deal with these!" comments
  const resolver = new DependencyResolver(processedInterfaces, dicts, enums);
  resolver.prepare();

  console.log(
    `Parsed ${dicts.size} dictionaries, ${processedInterfaces.size} interfaces, ${processedNamespaces.size} namespaces`
  );

  // Generate enums first (they're referenced by other types)
  console.log("Generating enums...");
  generateEnums();

  // Pre-populate dictOwner for all standalone dictionaries
  // This ensures systematic dictionary generation
  for (const dictName of resolver.dictOrdered) {
    dictOwner.set(dictName, `${dictName}.rs`);
  }

  // Generate standalone dictionaries in dependency order
  console.log("Generating standalone dictionaries...");
  let dictCount = 0;
  for (const dictName of resolver.dictOrdered) {
    const dict = dicts.get(dictName);
    if (dict) {
      generateDictionary(dictName, dict);
      dictCount++;
    }
  }
  console.log(`Generated ${dictCount} standalone dictionaries`);

  // Generate interfaces with proper dependency resolution
  console.log("Generating interfaces...");
  let interfaceCount = 0;
  for (const [interfaceName, interfaceRec] of processedInterfaces) {
    if (!interfaceRec.base && interfaceRec.partials.length === 0) {
      // Skip interfaces that are just partial definitions
      continue;
    }

    const dependencies = resolver.resolveInterfaceDependencies(
      interfaceName,
      interfaceRec.members
    );

    generateInterface(interfaceName, interfaceRec, dependencies);
    interfaceCount++;
  }
  console.log(`Generated ${interfaceCount} interfaces`);

  // Generate namespaces
  console.log("Generating namespaces...");
  let namespaceCount = 0;
  // eslint-disable-next-line no-unused-vars
  for (const [nsName, nsRec] of processedNamespaces) {
    const dependencies = resolver.resolveNamespaceDependencies(nsRec);
    generateNamespace(nsRec, dependencies);
    namespaceCount++;
  }
  console.log(`Generated ${namespaceCount} namespaces`);

  // Generate features.txt with WebIDL types and their parent dependencies
  console.log("Generating features.txt...");
  // generateFeatures(processedInterfaces, dicts, processedNamespaces);

  console.log("emlite-bind generation completed successfully!");
}

// /**
//  * Generate features.txt with WebIDL types and their parent dependencies
//  * @param {Map} interfaces - Processed interfaces
//  * @param {Map} dictionaries - Dictionary definitions
//  * @param {Map} namespaces - Processed namespaces
//  */
// function generateFeatures(interfaces, dictionaries, namespaces) {
//   const features = [];

//   // Process interfaces
//   for (const [name, interfaceRec] of interfaces) {
//     if (!interfaceRec.base && interfaceRec.partials.length === 0) {
//       continue; // Skip interfaces that are just partial definitions
//     }

//     const parent = interfaceRec.base?.inheritance;
//     if (parent && parent !== "emlite::Val") {
//       features.push(`${name} = ["${parent}"]`);
//     } else {
//       features.push(`${name} = []`);
//     }
//   }

//   // Process dictionaries
//   for (const [name, dict] of dictionaries) {
//     const parent = dict.inheritance;
//     if (parent && parent !== "emlite::Val") {
//       features.push(`${name} = ["${parent}"]`);
//     } else {
//       features.push(`${name} = []`);
//     }
//   }

//   // Process namespaces
//   for (const [name, namespace] of namespaces) {
//     features.push(`${name} = []`); // Namespaces don't have parents
//   }

//   // Sort features alphabetically
//   features.sort();

//   // Write to features.txt
//   const output = features.join("\n") + "\n";
//   writeFileSync("features.txt", output);
//   console.log(`Generated features.txt with ${features.length} WebIDL types`);
// }
