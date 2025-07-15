import fs from "fs";
import path from "path";
import * as changeCase from "change-case";
import { fileURLToPath } from "url";
import {
  builtinNominals,
  rustKeywords,
  missingDictFallback,
  IGNOREDFILES
} from "./ignored.js";
import { enums, typedefs, callbacks } from "./globals.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const OUT_SRC = path.resolve(__dirname, "../webbind/src");

function srcFile(name) {
    return path.join(OUT_SRC, `${name}.rs`);
}

export function writeSrcFile(name, srcLines) {
  const rs = srcFile(name);
  if (IGNOREDFILES.has(path.basename(rs))) return;
  const srcs = ["use super::*;\n\n", ...srcLines];
  fs.writeFileSync(rs, srcs.join("\n") + "\n", "utf8");
  //   console.log(`Parsed ${name}`);
}

export function flat(t) {
  if (!t) return { n: "__unk", unsigned: false };

  if (typeof t === "string") {
    let unsigned = false;
    let base = t.trim();

    base = base.replace(/^unrestricted /, "");

    if (base.startsWith("unsigned ")) {
      unsigned = true;
      base = base.replace(/^unsigned /, "");
    }
    return { n: base.trim(), unsigned };
  }

  if (Array.isArray(t)) return { n: "__union", unsigned: false };

  if (t.generic) {
    return {
      n: t.generic,
      inner: t.idlType,
      unsigned: false,
    };
  }

  if (t.idlType) {
    const inner = flat(t.idlType);
    return { n: inner.n, unsigned: !!t.unsigned || inner.unsigned };
  }

  return { n: "__unk", unsigned: false };
}

export function rust(idlType) {
  const { n, unsigned } = flat(idlType);
  const jsbindMap = {
    undefined: "Undefined",
    DOMString: "String",
    USVString: "String",
    ByteString: "String",
    CSSOMString: "String",
    Promise: "Promise",
    object: "Object",
    any: "Any",
    Uint8Array: "Uint8Array",
    Int8Array: "Int8Array",
    Uint32Array: "Uint32Array",
    Int32Array: "Int32Array",
    Float32Array: "Float32Array",
    Float64Array: "Float64Array",
    ArrayBuffer: "ArrayBuffer",
    DataView: "DataView",
  };

  if (missingDictFallback.has(n) || builtinNominals.has(n))
    return "Any";
  if (n.includes("EventInit")) return "Any";

  if (jsbindMap[n]) return jsbindMap[n];

  if (typeof idlType === "object" && idlType.generic) {
    const inner = idlType.idlType;

    const elem = Array.isArray(inner) ? inner[0] : inner;

    if (idlType.generic === "sequence") {
      return `Sequence<${rust(elem)}>`;
    }

    if (idlType.generic === "FrozenArray") {
      return `FrozenArray<${rust(elem)}>`;
    }

    if (idlType.generic === "ObservableArray") {
      return `ObservableArray<${rust(elem)}>`;
    }

    if (idlType.generic === "record") {
      const [k, v] = inner;
      return `Record<${rust(k)}, ${rust(v)}>`;
    }
  }
  if (["__union", "__unk"].includes(n)) return "Any";
  if (n === "boolean") return "bool";
  if (n === "byte") return "i8";
  if (n === "bigint") return "i64";
  if (n === "octet") return "u8";
  if (unsigned) {
    if (n === "short") return "u16";
    if (n === "long" || n === "int") return "u32";
    if (n === "long long") return "u64";
  } else {
    if (n === "short") return "i16";
    if (n === "long" || n === "int") return "i32";
    if (n === "long long") return "i64";
  }

  if (n === "double") return "f64";
  if (n === "float") return "f32";
  if (enums.has(n)) return n;
  if (callbacks.has(n)) return "Function";
  if (typedefs.has(n) || n === "__union") return "Any";

  return n;
}

const primitiveRE = /\b(?:i8|i16|i32|i64|u8|u16|u32|u64|bool|f32|f64)\b/;
export function argtypeFix(type) {
  if (type === "String") return "&str";
  return primitiveRE.test(type) ? type : `&${type}`;
}

export function fixIdent(name0) {
  const name = changeCase.snakeCase(name0);
  if (rustKeywords.has(name)) return name + "_";
  if (name === "BroadcastChannel" || name === "SharedWorker") return name + "_";
  if (name === "") return "none";
  let out = name.replace(/[/\-+]/g, "_");
  if (/^[0-9]/.test(out)) out = "_" + out;
  return out;
}

export function variantsOf(args) {
  const firstOpt = args.findIndex((a) => a.optional || a.default != null);
  if (firstOpt === -1) return [args];
  const out = [args.slice(0, firstOpt)];
  for (let i = firstOpt + 1; i <= args.length; ++i) out.push(args.slice(0, i));
  return out;
}

export function argDecl(args, withNames = true) {
  return args
    .map((a) => {
      const t = argtypeFix(rust(a.idlType));
      return withNames ? `${fixIdent(a.name)}: ${t}` : t;
    })
    .join(", ");
}
