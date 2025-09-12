import { writeSrcFile, rust, fixIdent, variantsOf, argDecl } from "../utils.js";

/**
 * Generate namespace functions as standalone Rust functions
 * @param {Object} namespace - Namespace definition
 * @param {Object} dependencies - Resolved dependencies (currently unused for namespaces)
 */
// eslint-disable-next-line no-unused-vars
export function generateNamespace(namespace, dependencies) {
  const src = [];

  // Group operations by name
  const groups = new Map();
  const order = [];
  namespace.members.filter((m) => m.type === "operation").forEach((op) => {
    if (!groups.has(op.name)) {
      groups.set(op.name, []);
      order.push(op.name);
    }
    groups.get(op.name).push(op);
  });

  const usedNames = new Set();
  const uniqueName = (candidate) => {
    let name = candidate;
    let n = 2;
    while (usedNames.has(name)) {
      name = `${candidate}_${n}`;
      n += 1;
    }
    usedNames.add(name);
    return name;
  };

  const withSuffix = (args) => {
    if (!args || args.length === 0) return "";
    const parts = args
      .map((a) => fixIdent(a.name))
      .map((n) => n.replace(/_+$/g, ""))
      .filter((n) => n.length > 0);
    if (parts.length === 0) return "";
    const joined = parts.join("_and_");
    const cleaned = joined.replace(/_+/g, "_");
    return `with_${cleaned}`;
  };

  for (const name of order) {
    const ops = groups.get(name);
    const total = ops.reduce((sum, op) => sum + variantsOf(op.arguments).length, 0);
    for (const op of ops) {
      const ret = rust(op.idlType || "undefined");
      const base = fixIdent(op.name);
      const variants = variantsOf(op.arguments);
      const baseLen = variants[0]?.length || 0;
      const varNames = variants.map((args, idx) => {
        if (total === 1 && variants.length === 1) return uniqueName(base);
        if (idx === 0) {
          const candidate = uniqueName(base);
          if (candidate === base) return candidate;
          const suf0 = withSuffix(args);
          const join0 = base.endsWith("_") || !suf0 ? "" : "_";
          const fallback = suf0 ? `${base}${join0}${suf0}` : `${base}_2`;
          return uniqueName(fallback);
        }
        const extras = args.slice(baseLen);
        const suf = withSuffix(extras);
        const join = base.endsWith("_") || !suf ? "" : "_";
        const candidate = suf ? `${base}${join}${suf}` : `${base}_2`;
        return uniqueName(candidate);
      });

      let idx = 0;
      for (const v of variants) {
        const declSrc = argDecl(v);
        const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");
        const callExpr = `Any::global("${namespace.name}").call("${op.name}", &[${
          callArgs ? callArgs + ", " : ""
        }])`;
        src.push(
          `/// The ${op.name} function from the ${namespace.name} namespace.`,
          `pub fn ${varNames[idx]}(${declSrc}) -> ${ret} {`,
          `    ${callExpr}.as_::<${ret}>()`,
          `}`,
          ""
        );
        idx += 1;
      }
    }
  }

  writeSrcFile(namespace.name, src);
}
