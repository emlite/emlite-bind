import { writeSrcFile } from "../utils.js";
import { emittedDicts, dictOwner } from "../globals.js";

/**
 * Generate member method implementations for a dictionary
 * @param {Object} dict - Dictionary definition
 * @param {Array} src - Source lines array
 */
function generateDictionaryMethods(dict, src) {
  dict.members.forEach((m) => {
    const S = emitAttr(m, dict.name);
    src.push(...S);
  });
}

/**
 * Generate attribute getter/setter methods for dictionaries
 * Includes documentation comments matching the enhanced wasmbind pattern
 * @param {Object} attr - Attribute definition
 * @param {string} owner - Owner type name
 * @returns {Array<string>} Generated source lines
 */
function emitAttr(attr, owner) {
  if (attr.name.includes("-")) return [];
  
  const S = [`impl ${owner} {`];
  const type = rust(attr.idlType);
  
  // Getter method with documentation
  S.push(
    `    /// Getter of the \`${attr.name}\` attribute.`,
    `    pub fn ${fixIdent(attr.name)}(&self) -> ${type} {`,
    `        self.inner.get("${attr.name}").as_::<${type}>()`,
    `    }`,
    ""
  );

  // Setter method with documentation (if not readonly)
  if (!attr.readonly) {
    S.push(
      `    /// Setter of the \`${attr.name}\` attribute.`,
      `    pub fn set_${fixIdent(attr.name)}(&mut self, value: ${argtypeFix(type)}) {`,
      `        self.inner.set("${attr.name}", value);`,
      `    }`
    );
  }
  
  S.push("}");
  return S;
}

/**
 * Generate complete dictionary structure with all traits
 * @param {Object} dict - Dictionary definition
 * @param {Array} src - Source lines array
 * @param {string} ownerName - Owner name for tracking
 */
export function embedDict(dict, src, ownerName) {
  if (emittedDicts.has(dict.name)) return;
  emittedDicts.add(dict.name);
  
  if (!dictOwner.has(dict.name)) {
    dictOwner.set(dict.name, `${ownerName}.rs`);
  }
  
  // Generate the main struct
  src.push(
    `/// The ${dict.name} dictionary.`,
    `#[derive(Clone, Debug, PartialEq, PartialOrd)]`,
    `#[repr(transparent)]`,
    `pub struct ${dict.name} {`,
    `    inner: Any,`,
    `}`,
    ""
  );
  
  // Generate FromVal trait implementation
  src.push(
    `impl FromVal for ${dict.name} {`,
    `    fn from_val(v: &Any) -> Self {`,
    `        ${dict.name} { inner: v.clone() }`,
    `    }`,
    `    fn take_ownership(v: AnyHandle) -> Self {`,
    `        Self::from_val(&Any::take_ownership(v))`,
    `    }`,
    `    fn as_handle(&self) -> AnyHandle {`,
    `        self.inner.as_handle()`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate Deref trait implementation
  src.push(
    `impl core::ops::Deref for ${dict.name} {`,
    `    type Target = Any;`,
    `    fn deref(&self) -> &Self::Target {`,
    `        &self.inner`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate DerefMut trait implementation
  src.push(
    `impl core::ops::DerefMut for ${dict.name} {`,
    `    fn deref_mut(&mut self) -> &mut Self::Target {`,
    `        &mut self.inner`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate AsRef trait implementation
  src.push(
    `impl AsRef<Any> for ${dict.name} {`,
    `    fn as_ref(&self) -> &Any {`,
    `        &self.inner`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate AsMut trait implementation
  src.push(
    `impl AsMut<Any> for ${dict.name} {`,
    `    fn as_mut(&mut self) -> &mut Any {`,
    `      &mut self.inner`,
    `  }`,
    `}`,
    ""
  );
  
  // Generate From<Dict> for Any implementation
  src.push(
    `impl From<${dict.name}> for Any {`,
    `    fn from(s: ${dict.name}) -> Any {`,
    `        let handle = s.inner.as_handle();`,
    `        core::mem::forget(s);`,
    `        Any::take_ownership(handle)`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate From<&Dict> for Any implementation
  src.push(
    `impl From<&${dict.name}> for Any {`,
    `    fn from(s: &${dict.name}) -> Any {`,
    `        s.inner.clone()`,
    `    }`,
    `}`,
    ""
  );
  
  // Generate dictionary member methods
  generateDictionaryMethods(dict, src);
}

/**
 * Generate standalone dictionary file
 * @param {string} dictName - Dictionary name
 * @param {Object} dict - Dictionary definition
 */
export function generateDictionary(dictName, dict) {
  const src = ["\n"];
  embedDict(dict, src, dictName);
  writeSrcFile(dictName, src);
}

// Import required utilities that were missing
import { rust, fixIdent, argtypeFix } from "../utils.js";