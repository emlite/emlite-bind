import { writeSrcFile, rust, fixIdent, argtypeFix, variantsOf, argDecl } from "../utils.js";
import { embedDict } from "./dictionary-generator.js";

/**
 * Generate attribute getter/setter methods for interfaces
 * @param {Object} attr - Attribute definition
 * @param {string} owner - Owner interface name
 * @param {boolean} isStatic - Whether the attribute is static
 * @param {boolean} isInterface - Whether this is for an interface (adds docs)
 * @returns {Array<string>} Generated source lines
 */
function emitAttr(attr, owner, isStatic = false, isInterface = false) {
  if (attr.name.includes("-")) return [];
  const S = [`impl ${owner} {`];
  const type = rust(attr.idlType);
  
  if (isStatic) {
    if (isInterface)
      S.push(
        `    /// Getter of the \`${attr.name}\` static attribute.`,
        `    /// [\`${owner}.${attr.name}\`](https://developer.mozilla.org/en-US/docs/Web/API/${owner}/${attr.name})`
      );
    S.push(
      `    pub fn ${fixIdent(attr.name)}() -> ${type} {`,
      `        Any::global("${owner}").get("${attr.name}").as_::<${type}>()`,
      `    }`,
      ""
    );
  } else {
    if (isInterface)
      S.push(
        `    /// Getter of the \`${attr.name}\` attribute.`,
        `    /// [\`${owner}.${attr.name}\`](https://developer.mozilla.org/en-US/docs/Web/API/${owner}/${attr.name})`
      );
    S.push(
      `    pub fn ${fixIdent(attr.name)}(&self) -> ${type} {`,
      `        self.inner.get("${attr.name}").as_::<${type}>()`,
      `    }`,
      ""
    );

    if (!attr.readonly) {
      if (isInterface)
        S.push(
          `    /// Setter of the \`${attr.name}\` attribute.`,
          `    /// [\`${owner}.${attr.name}\`](https://developer.mozilla.org/en-US/docs/Web/API/${owner}/${attr.name})`
        );
      S.push(
        `    pub fn set_${fixIdent(attr.name)}(&mut self, value: ${argtypeFix(type)}) {`,
        `        self.inner.set("${attr.name}", value);`,
        `    }`
      );
    }
  }
  S.push("}");
  return S;
}

/**
 * Generate operation methods for interfaces
 * @param {Object} op - Operation definition
 * @param {string} owner - Owner interface name
 * @param {boolean} isStatic - Whether the operation is static
 * @param {boolean} isInterface - Whether this is for an interface (adds docs)
 * @returns {Array<string>} Generated source lines
 */
function emitOp(op, owner, isStatic = false, isInterface = false, variantNames) {
  const S = [];
  if (!op.name) return { S: [], used: 0 };
  const variants = variantsOf(op.arguments);
  const ret = rust(op.idlType || "undefined");

  const names = Array.isArray(variantNames) && variantNames.length === variants.length
    ? variantNames
    : variants.map((_, i) => `${fixIdent(op.name)}${i === 0 ? "" : i}`);

  let idx = 0;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");
    const callExpr = isStatic
      ? `Any::global("${owner}").call("${op.name}", &[${callArgs ? callArgs + ", " : ""}])`
      : `self.inner.call("${op.name}", &[${callArgs ? callArgs + ", " : ""}])`;

    S.push(`impl ${owner} {`);
    if (isInterface)
      S.push(
        `    /// The ${op.name} method.`,
        `    /// [\`${owner}.${op.name}\`](https://developer.mozilla.org/en-US/docs/Web/API/${owner}/${op.name})`
      );
    S.push(
      `    pub fn ${names[idx]}(${isStatic ? "" : "&self, "}${declSrc}) -> ${ret} {`,
      `        ${callExpr}.as_::<${ret}>()`,
      `    }`,
      `}`
    );
    idx += 1;
  }
  return { S, used: variants.length };
}

/**
 * Generate constructor methods for interfaces
 * @param {Object} ctor - Constructor definition
 * @param {string} owner - Owner interface name
 * @param {string} parent - Parent class name
 * @returns {Array<string>} Generated source lines
 */
function emitCtor(ctor, owner, parent) {
  const variants = variantsOf(ctor.arguments);
  const S = [`\nimpl ${owner} {`];

  let i = 0;
  const sz = variants.length;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");

    S.push(
      `    /// The \`new ${owner}(..)\` constructor, creating a new ${owner} instance`,
      `    pub fn new${sz === 1 ? "" : i}(${declSrc}) -> ${owner} {
        Self {
            inner: Any::global("${owner}").new(&[${callArgs}]).as_::<${parent}>(),
        }
    }`,
      ""
    );
    i += 1;
  }
  S.push("}");
  return S;
}

function emitCtorGrouped(
  ctor,
  owner,
  parent,
  groupTotal = undefined,
  suffixStart = 0
) {
  const variants = variantsOf(ctor.arguments);
  const S = [`\nimpl ${owner} {`];

  let i = suffixStart;
  const sz = variants.length;
  const totalGroup = typeof groupTotal === "number" ? groupTotal : sz;
  const omitSuffix = totalGroup === 1 ? true : sz === 1 && suffixStart === 0;

  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");

    S.push(
      `    /// The \`new ${owner}(..)\` constructor, creating a new ${owner} instance`,
      `    pub fn new${omitSuffix ? "" : i}(${declSrc}) -> ${owner} {\n        Self {\n            inner: Any::global("${owner}").new(&[${callArgs}]).as_::<${parent}>(),\n        }\n    }`,
      ""
    );
    i += 1;
  }
  S.push("}");
  return { S, used: variants.length };
}

// New helper: emit constructors with explicit variant names
function emitCtorNamed(ctor, owner, parent, variantNames) {
  const variants = variantsOf(ctor.arguments);
  const S = [];
  const names = Array.isArray(variantNames) && variantNames.length === variants.length
    ? variantNames
    : variants.map((_, i) => (i === 0 ? "new" : `new${i}`));

  let idx = 0;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");

    S.push(
      `\nimpl ${owner} {`,
      `    /// The \`new ${owner}(..)\` constructor, creating a new ${owner} instance`,
      `    pub fn ${names[idx]}(${declSrc}) -> ${owner} {`,
      `        Self {`,
      `            inner: Any::global("${owner}").new(&[${callArgs}]).as_::<${parent}>(),`,
      `        }`,
      `    }`,
      `}`,
      ""
    );
    idx += 1;
  }
  return { S, used: variants.length };
}

/**
 * Generate complete interface with all members and dependencies
 * @param {string} interfaceName - Name of the interface
 * @param {Object} interfaceRec - Interface record with members and metadata
 * @param {Object} dependencies - Resolved dependencies from DependencyResolver
 */
export function generateInterface(interfaceName, interfaceRec, dependencies) {
  const src = ["\n"];
  let parent = interfaceRec.base?.inheritance || null;

  // eslint-disable-next-line no-unused-vars
  const { localEnums, localDicts } = dependencies;

  // Embed local dictionaries
  localDicts.forEach((d) => embedDict(d, src, interfaceName));

  // Set default parent if none specified
  if (!parent) parent = "Any";

  // Generate the main interface struct
  src.push(
    `/// The ${interfaceName} class.`,
    `/// [\`${interfaceName}\`](https://developer.mozilla.org/en-US/docs/Web/API/${interfaceName})`,
    `#[derive(Clone, Debug, PartialEq, PartialOrd)]`,
    `#[repr(transparent)]`,
    `pub struct ${interfaceName} {`,
    `    inner: ${parent},`,
    `}`,
    ""
  );

  // Generate FromVal trait implementation
  src.push(
    `impl FromVal for ${interfaceName} {`,
    `    fn from_val(v: &Any) -> Self {`,
    `        ${interfaceName} { inner: ${parent}::from_val(v) }`,
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
    `impl core::ops::Deref for ${interfaceName} {`,
    `    type Target = ${parent};`,
    `    fn deref(&self) -> &Self::Target {`,
    `        &self.inner`,
    `    }`,
    `}`,
    ""
  );

  // Generate DerefMut trait implementation
  src.push(
    `impl core::ops::DerefMut for ${interfaceName} {`,
    `    fn deref_mut(&mut self) -> &mut Self::Target {`,
    `        &mut self.inner`,
    `    }`,
    `}`,
    ""
  );

  // Generate AsRef trait implementation
  src.push(
    `impl AsRef<Any> for ${interfaceName} {`,
    `    fn as_ref(&self) -> &Any {`,
    `        &self.inner`,
    `    }`,
    `}`,
    ""
  );

  // Generate AsMut trait implementation
  src.push(
    `impl AsMut<Any> for ${interfaceName} {`,
    `    fn as_mut(&mut self) -> &mut Any {`,
    `      &mut self.inner`,
    `  }`,
    `}`,
    ""
  );

  // Generate From<Interface> for Any implementation
  src.push(
    `impl From<${interfaceName}> for Any {`,
    `    fn from(s: ${interfaceName}) -> Any {`,
    `        let handle = s.inner.as_handle();`,
    `        core::mem::forget(s);`,
    `        Any::take_ownership(handle)`,
    `    }`,
    `}`,
    ""
  );

  // Generate From<&Interface> for Any implementation
  src.push(
    `impl From<&${interfaceName}> for Any {`,
    `    fn from(s: &${interfaceName}) -> Any {`,
    `        s.inner.clone().into()`,
    `    }`,
    `}`,
    ""
  );

  // Generate dynamic cast utility
  src.push(`jsbind::utils::impl_dyn_cast!(${interfaceName});`, "");
  src.push("");

  // Generate constants
  interfaceRec.consts.forEach((c) =>
    src.push(`    const ${c.name}: ${rust(c.idlType)} = ${c.value.value};`)
  );

  // Generate member methods
  const usedNames = new Set();
  // First emit attributes and collect constructors directly
  const opGroups = new Map();
  const opOrder = [];
  const ctors = [];
  interfaceRec.members.forEach((m) => {
    const isStatic = m.static === true || m.special === "static";
    if (m.type === "attribute") {
      const S = emitAttr(m, interfaceName, isStatic, true);
      src.push(...S);
      usedNames.add(fixIdent(m.name));
      if (!m.readonly) usedNames.add(`set_${fixIdent(m.name)}`);
    } else if (
      m.type === "constructor" ||
      (m.type === "operation" && m.special === "constructor")
    ) {
      ctors.push(m);
    } else if (m.type === "operation") {
      const key = `${isStatic ? "static:" : ""}${m.name}`;
      if (!opGroups.has(key)) {
        opGroups.set(key, { isStatic, ops: [] });
        opOrder.push(key);
      }
      opGroups.get(key).ops.push(m);
    }
  });

  // Unique name helper with conflict fallback (and _static for static)
  const uniqueName = (candidate, isStatic = false) => {
    let name = candidate;
    if (usedNames.has(name) && isStatic && !name.endsWith("_static")) {
      name = `${name}_static`;
    }
    let n = 2;
    while (usedNames.has(name)) {
      name = `${candidate}_${n}`;
      n += 1;
    }
    usedNames.add(name);
    return name;
  };

  // Build a sanitized "with_..." suffix from parameter names
  const withSuffix = (args) => {
    if (!args || args.length === 0) return "";
    const parts = args
      .map((a) => fixIdent(a.name))
      // drop trailing underscores from keyword-renamed params like type_ / async_
      .map((n) => n.replace(/_+$/g, ""))
      .filter((n) => n.length > 0);
    if (parts.length === 0) return "";
    const joined = parts.join("_and_");
    // collapse any accidental multiple underscores
    const cleaned = joined.replace(/_+/g, "_");
    return `with_${cleaned}`;
  };

  // Emit constructors with param-name suffixes (extras-only per constructor)
  if (ctors.length > 0) {
    const totalCtorVariants = ctors.reduce(
      (sum, c) => sum + variantsOf(c.arguments).length,
      0
    );
    for (const c of ctors) {
      const variants = variantsOf(c.arguments);
      const baseLen = variants[0]?.length || 0;
      const names = variants.map((args, idx) => {
        // Base variant for the first constructor seen gets plain `new` if available
        if (idx === 0) {
          const baseName = uniqueName("new");
          if (baseName === "new") return baseName;
          // If `new` was already taken by a previous constructor, include full params
          const suf = withSuffix(args);
          const candidate = suf ? `new_${suf}` : "new_2";
          return uniqueName(candidate);
        }
        // Extras-only suffix for additional optional params relative to base of this constructor
        const extras = args.slice(baseLen);
        const suf = withSuffix(extras);
        const candidate = suf ? `new_${suf}` : "new_2";
        return uniqueName(candidate);
      });
      const { S } = emitCtorNamed(c, interfaceName, parent, names);
      src.push(...S);
    }
  }

  // Emit operations grouped by name and static-ness using param-name suffixes
  for (const key of opOrder) {
    const { isStatic, ops } = opGroups.get(key);
    const total = ops.reduce((sum, op) => sum + variantsOf(op.arguments).length, 0);
    for (const op of ops) {
      const variants = variantsOf(op.arguments);
      const base = fixIdent(op.name);
      const baseLen = variants[0]?.length || 0;
      const names = variants.map((args, idx) => {
        if (total === 1 && variants.length === 1) return uniqueName(base, isStatic);
        if (idx === 0) {
          // Base variant for this op: use plain base if available, else include full params
          const candidate = uniqueName(base, isStatic);
          if (candidate === base) return candidate;
          const suf0 = withSuffix(args);
          const join0 = base.endsWith("_") || !suf0 ? "" : "_";
          const fallback = suf0 ? `${base}${join0}${suf0}` : `${base}_2`;
          return uniqueName(fallback, isStatic);
        }
        const extras = args.slice(baseLen);
        const suf = withSuffix(extras);
        const join = base.endsWith("_") || !suf ? "" : "_";
        const candidate = suf ? `${base}${join}${suf}` : `${base}_2`;
        return uniqueName(candidate, isStatic);
      });
      const { S } = emitOp(op, interfaceName, isStatic, true, names);
      src.push(...S);
    }
  }

  writeSrcFile(interfaceName, src);
}
