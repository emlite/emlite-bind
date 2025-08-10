import { writeSrcFile, rust, fixIdent, variantsOf, argDecl } from "../utils.js";

/**
 * Generate namespace functions as standalone Rust functions
 * @param {Object} namespace - Namespace definition
 * @param {Object} dependencies - Resolved dependencies (currently unused for namespaces)
 */
// eslint-disable-next-line no-unused-vars
export function generateNamespace(namespace, dependencies) {
  const src = [];

  namespace.members
    .filter((m) => m.type === "operation")
    .forEach((op) => {
      const ret = rust(op.idlType || "undefined");
      const rustName = fixIdent(op.name);
      const variants = variantsOf(op.arguments);
      let i = 0;
      const sz = variants.length;
      
      for (const v of variants) {
        const declSrc = argDecl(v);
        const callArgs = v
          .map((a) => `${fixIdent(a.name)}.into()`)
          .join(", ");

        const callExpr = `Any::global("${namespace.name}").call("${op.name}", &[${
          callArgs ? callArgs + ", " : ""
        }])`;

        src.push(
          `/// The ${op.name} function from the ${namespace.name} namespace.`,
          `pub fn ${rustName}${sz === 1 ? "" : i}(${declSrc}) -> ${ret} {`,
          `    ${callExpr}.as_::<${ret}>()`,
          `}`,
          ""
        );
        i += 1;
      }
    });
    
  writeSrcFile(namespace.name, src);
}