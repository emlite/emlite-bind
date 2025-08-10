import { enums } from "../globals.js";
import { writeSrcFile, fixIdent } from "../utils.js";

/**
 * Generate Rust enum definitions from WebIDL enum specifications
 */
export function generateEnums() {
  const src = ["\n"];
  
  for (const e of enums.values()) {
    src.push(
      `#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]`,
      `pub enum ${e.name} {`
    );
    
    for (const v of e.values) {
      src.push(`    ${fixIdent(v.value).toUpperCase()},`);
    }
    
    src.push("}");
    
    // Generate FromVal implementation
    src.push(
      `impl FromVal for ${e.name} {`,
      `    fn from_val(v: &Any) -> Self {`,
      `         match v.as_::<Option<&str>>().unwrap() {`
    );
    
    for (const v of e.values) {
      src.push(
        `            "${v.value}" => Self::${fixIdent(v.value).toUpperCase()},`
      );
    }
    
    src.push(
      `             _ => unreachable!(),`,
      `        }`,
      `    }`,
      `    fn take_ownership(v: AnyHandle) -> Self {`,
      `        Self::from_val(&Any::take_ownership(v))`,
      `    }`,
      `    fn as_handle(&self) -> AnyHandle {`,
      `        Any::from(*self).as_handle()`,
      `    }`,
      `}`
    );
    
    // Generate From<Enum> for Any implementation
    src.push(
      `impl From<${e.name}> for Any {`,
      `    fn from(s: ${e.name}) -> Any {`,
      `         match s {`
    );
    
    for (const v of e.values) {
      src.push(
        `            ${e.name}::${fixIdent(v.value).toUpperCase()} => Any::from("${v.value}"),`
      );
    }
    
    src.push("         }");
    src.push(
      `    }`,
      `}`,
      `impl From<&${e.name}> for Any {`,
      `    fn from(s: &${e.name}) -> Any {`,
      `         match *s {`
    );
    
    for (const v of e.values) {
      src.push(
        `            ${e.name}::${fixIdent(v.value).toUpperCase()} => Any::from("${v.value}"),`
      );
    }
    
    src.push("         }");
    src.push(`    }`, `}`, "");
    src.push("");
  }

  writeSrcFile("enums", src);
}