import { getAllTypeRefs } from "./type-analyzer.js";
import { dictOwner } from "../globals.js";

/**
 * Systematic dependency resolution for Rust code generation
 * Replaces ad-hoc "not sure how to deal with these!" comments with organized approach
 */
export class DependencyResolver {
  constructor(interfaces, dicts, enums, callbackInterfaces) {
    this.interfaces = interfaces;
    this.dicts = dicts;
    this.enums = enums;
    this.callbackInterfaces = callbackInterfaces;
    this.dictDepGraph = new Map();
    this.dictOrdered = [];
  }

  /**
   * Build dependency graph for dictionaries to ensure proper generation order
   */
  buildDictionaryDependencyGraph() {
    for (const [dictName, dict] of this.dicts) {
      const deps = new Set();
      const refs = getAllTypeRefs(dict.members);

      refs.forEach((ref) => {
        if (this.dicts.has(ref) && ref !== dictName) {
          deps.add(ref);
        }
      });

      // Add inheritance dependencies
      if (dict.inheritance && this.dicts.has(dict.inheritance)) {
        deps.add(dict.inheritance);
      }

      this.dictDepGraph.set(dictName, deps);
    }
  }

  /**
   * Topologically sort dictionaries to resolve generation order
   * @returns {Array<string>} Dictionary names in dependency order
   */
  topologicalSortDictionaries() {
    const visited = new Set();
    const temp = new Set();
    const result = [];

    const visit = (dict) => {
      if (temp.has(dict)) {
        console.warn(
          `Circular dependency detected involving dictionary: ${dict}`
        );
        return;
      }
      if (visited.has(dict)) return;

      temp.add(dict);
      const deps = this.dictDepGraph.get(dict) || new Set();

      for (const dep of deps) {
        if (this.dicts.has(dep)) {
          visit(dep);
        }
      }

      temp.delete(dict);
      visited.add(dict);
      result.push(dict);
    };

    for (const dictName of this.dicts.keys()) {
      if (!visited.has(dictName)) {
        visit(dictName);
      }
    }

    this.dictOrdered = result;
    return result;
  }

  /**
   * Resolve dependencies for interface generation
   * Systematically handles types that were previously marked with uncertainty
   * @param {string} interfaceName - Name of the interface
   * @param {Array} members - Interface members array
   * @returns {Object} Organized dependency information
   */
  resolveInterfaceDependencies(interfaceName, members) {
    const externalTypeRefs = new Set();
    const localEnums = new Set();
    const localDicts = [];
    const processedDictRefs = new Set();

    const refs = getAllTypeRefs(members);

    refs.forEach((ref) => {
      // Handle dictionary references
      if (this.dicts.has(ref)) {
        if (!dictOwner.has(ref)) {
          // Dictionary not yet owned - embed locally
          const dict = this.dicts.get(ref);
          if (dict && !localDicts.some((d) => d.name === ref)) {
            localDicts.push(dict);
            processedDictRefs.add(ref);
          }
        } else {
          // Dictionary owned elsewhere - external reference
          externalTypeRefs.add(ref);
        }
        return;
      }

      // Handle enum references
      if (this.enums.has(ref)) {
        localEnums.add(this.enums.get(ref));
        return;
      }

      // Handle interface references
      if (this.interfaces.has(ref)) {
        externalTypeRefs.add(ref);
        return;
      }

      // Handle callback interface references
      if (this.callbackInterfaces && this.callbackInterfaces.has(ref)) {
        externalTypeRefs.add(ref);
      }
    });

    // Recursively resolve dependencies for local dictionaries
    localDicts.forEach((dict) => {
      this.addLocalDictDependencies(dict.name, localDicts, processedDictRefs);
    });

    // Assign ownership of local dictionaries to this interface
    this.assignDictionaryOwnership(interfaceName, localDicts);

    return {
      externalTypeRefs,
      localEnums,
      localDicts,
    };
  }

  /**
   * Add local dictionary dependencies recursively
   * @param {string} dictName - Dictionary name to process
   * @param {Array} localDicts - Array of local dictionaries to add to
   * @param {Set} processedRefs - Set of already processed references
   */
  addLocalDictDependencies(dictName, localDicts, processedRefs) {
    const dict = this.dicts.get(dictName);
    if (!dict) return;

    const refs = getAllTypeRefs(dict.members);
    refs.forEach((ref) => {
      if (
        this.dicts.has(ref) &&
        !processedRefs.has(ref) &&
        !dictOwner.has(ref)
      ) {
        const depDict = this.dicts.get(ref);
        if (depDict && !localDicts.some((d) => d.name === ref)) {
          localDicts.push(depDict);
          processedRefs.add(ref);
          // Recursively add dependencies
          this.addLocalDictDependencies(ref, localDicts, processedRefs);
        }
      }
    });
  }

  /**
   * Assign ownership of local dictionaries to the specified interface
   * @param {string} interfaceName - Name of the owning interface
   * @param {Array} localDicts - Array of dictionaries to assign
   */
  assignDictionaryOwnership(interfaceName, localDicts) {
    localDicts.forEach((dict) => {
      if (!dictOwner.has(dict.name)) {
        dictOwner.set(dict.name, `${interfaceName}.rs`);
      }
    });
  }

  /**
   * Resolve dependencies for namespace generation
   * @param {Object} namespace - Namespace object
   * @returns {Object} Dependency information for namespace
   */
  resolveNamespaceDependencies(namespace) {
    const externalTypeRefs = new Set();

    const refs = getAllTypeRefs(namespace.members);

    refs.forEach((ref) => {
      // For namespaces, we mainly need external type references
      if (this.dicts.has(ref) || this.interfaces.has(ref)) {
        externalTypeRefs.add(ref);
        return;
      }
      if (this.callbackInterfaces && this.callbackInterfaces.has(ref)) {
        externalTypeRefs.add(ref);
      }
    });

    return {
      externalTypeRefs,
      localEnums: new Set(),
      localDicts: [],
    };
  }

  /**
   * Initialize the dependency resolver
   * Call this before using other resolution methods
   */
  prepare() {
    this.buildDictionaryDependencyGraph();
    this.topologicalSortDictionaries();
  }
}
