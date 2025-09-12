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
function emitOp(
  op,
  owner,
  isStatic = false,
  isInterface = false,
  groupTotal = undefined,
  suffixStart = 0
) {
  const S = [];
  if (!op.name) return { S: [], used: 0 };
  S.push(`impl ${owner} {`);
  const ret = rust(op.idlType || "undefined");
  const rustName = fixIdent(op.name);
  const variants = variantsOf(op.arguments);

  let i = suffixStart;
  const sz = variants.length;
  const totalGroup = typeof groupTotal === "number" ? groupTotal : sz;
  const omitSuffix = totalGroup === 1 ? true : sz === 1 && suffixStart === 0;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");
    const callExpr = isStatic
      ? `Any::global("${owner}").call("${op.name}", &[${
          callArgs ? callArgs + ", " : ""
        }])`
      : `self.inner.call("${op.name}", &[${callArgs ? callArgs + ", " : ""}])`;
    if (isInterface)
      S.push(
        `    /// The ${op.name} method.`,
        `    /// [\`${owner}.${op.name}\`](https://developer.mozilla.org/en-US/docs/Web/API/${owner}/${op.name})`
      );
    S.push(
      `    pub fn ${rustName}${omitSuffix ? "" : i}(${
        isStatic ? "" : "&self, "
      }${declSrc}) -> ${ret} {`,
      `        ${callExpr}.as_::<${ret}>()`,
      `    }`
    );
    i += 1;
  }
  S.push("}");
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
  // First emit attributes and collect constructors directly
  const opGroups = new Map();
  const opOrder = [];
  const ctors = [];
  interfaceRec.members.forEach((m) => {
    const isStatic = m.static === true || m.special === "static";
    if (m.type === "attribute") {
      const S = emitAttr(m, interfaceName, isStatic, true);
      src.push(...S);
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

  // Emit constructors grouped by total overload count with consistent suffix numbering
  if (ctors.length > 0) {
    const totalCtorVariants = ctors.reduce(
      (sum, c) => sum + variantsOf(c.arguments).length,
      0
    );
    let offset = 0;
    for (const c of ctors) {
      const { S, used } = emitCtorGrouped(
        c,
        interfaceName,
        parent,
        totalCtorVariants,
        offset
      );
      src.push(...S);
      offset += used;
    }
  }

  // Emit operations grouped by name and static-ness with consistent suffix numbering
  for (const key of opOrder) {
    const { isStatic, ops } = opGroups.get(key);
    const total = ops.reduce((sum, op) => sum + variantsOf(op.arguments).length, 0);
    let offset = 0;
    for (const op of ops) {
      const { S, used } = emitOp(op, interfaceName, isStatic, true, total, offset);
      src.push(...S);
      offset += used;
    }
  }

  writeSrcFile(interfaceName, src);
}
