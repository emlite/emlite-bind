import { parseSpecs, processInterfaces } from "./parsers/spec-parser.js";
import { DependencyResolver } from "./parsers/dependency-resolver.js";
import { generateEnums } from "./generators/enum-generator.js";
import { generateNamespace } from "./generators/namespace-generator.js";
import { generateInterface } from "./generators/interface-generator.js";
import { generateDictionary } from "./generators/dictionary-generator.js";
import { enums, dictOwner } from "./globals.js";

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

  // Create dependency resolver to systematically handle type dependencies
  // This replaces the "not sure how to deal with these!" comments
  const resolver = new DependencyResolver(processedInterfaces, dicts, enums);
  resolver.prepare();

  console.log(`Parsed ${dicts.size} dictionaries, ${processedInterfaces.size} interfaces, ${namespaces.length} namespaces`);

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
  for (const ns of namespaces) {
    const dependencies = resolver.resolveNamespaceDependencies(ns);
    generateNamespace(ns, dependencies);
    namespaceCount++;
  }
  console.log(`Generated ${namespaceCount} namespaces`);

  console.log("emlite-bind generation completed successfully!");
}