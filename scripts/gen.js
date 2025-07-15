import {
  flat,
  rust,
  writeSrcFile,
  fixIdent,
  argtypeFix,
  variantsOf,
  argDecl,
} from "./utils.js";
import {
  emittedDicts,
  dictOwner,
  enums,
  typedefs,
  callbacks,
} from "./globals.js";

function emitAttr(attr, owner, isStatic = false) {
  const S = [`impl ${owner} {`];
  const type = rust(attr.idlType);

  if (isStatic) {
    S.push(
      `    pub fn ${fixIdent(attr.name)}() -> ${type} {`,
      `        emlite::Val::global("${owner}").get("${
        attr.name
      }").as_::<${type}>()`,
      `    }`,
      ""
    );
  } else {
    S.push(
      `    pub fn ${fixIdent(attr.name)}(&self) -> ${type} {`,
      `        self.inner.get("${attr.name}").as_::<${type}>()`,
      `    }`,
      ""
    );

    if (!attr.readonly) {
      S.push(
        `    pub fn set_${fixIdent(attr.name)}(&mut self, value: ${argtypeFix(
          type
        )}) {`,
        `        self.inner.set("${attr.name}", value);`,
        `    }`,
        ""
      );
    }
  }
  S.push("}");
  return S;
}

function emitOp(op, owner, isStatic = false) {
  const S = [];
  if (!op.name) return S;
  S.push(`impl ${owner} {`);
  const ret = rust(op.idlType || "undefined");
  const rustName = fixIdent(op.name);
  const variants = variantsOf(op.arguments);

  let i = 0;
  const sz = variants.length;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");
    const callExpr = isStatic
      ? `emlite::Val::global("${owner}").call("${op.name}", &[${
          callArgs ? callArgs + ", " : ""
        }])`
      : `self.inner.call("${op.name}", &[${callArgs ? callArgs + ", " : ""}])`;

    S.push(
      `    pub fn ${rustName}${sz === 1 ? "" : i}(${
        isStatic ? "" : "&self, "
      }${declSrc}) -> ${ret} {`,
      `        ${callExpr}.as_::<${ret}>()`,
      `    }`,
      ""
    );
    i += 1;
  }
  S.push("}");
  return S;
}

function emitCtor(ctor, owner, parent) {
  const variants = variantsOf(ctor.arguments);
  const S = [`\nimpl ${owner} {`];

  let i = 0;
  const sz = variants.length;
  for (const v of variants) {
    const declSrc = argDecl(v);
    const callArgs = v.map((a) => `${fixIdent(a.name)}.into()`).join(", ");

    S.push(
      `    pub fn new${sz === 1 ? "" : i}(${declSrc}) -> ${owner} {
        Self {
            inner: emlite::Val::global("${owner}").new(&[${callArgs}]).as_::<${parent}>(),
        }
    }`,
      ""
    );
    i += 1;
  }
  S.push("}");
  return S;
}

function embedDict(dict, src, ownerName) {
  if (emittedDicts.has(dict.name)) return;
  emittedDicts.add(dict.name);
  if (!dictOwner.has(dict.name)) {
    dictOwner.set(dict.name, `${ownerName}.rs`);
  }
  src.push(
    `#[derive(Clone, Debug, PartialEq, PartialOrd)]`,
    `#[repr(transparent)]`,
    `pub struct ${dict.name} {`,
    `    inner: emlite::Val,`,
    `}`,
    `impl FromVal for ${dict.name} {`,
    `    fn from_val(v: &emlite::Val) -> Self {`,
    `        ${dict.name} { inner: v.clone() }`,
    `    }`,
    `    fn take_ownership(v: emlite::env::Handle) -> Self {`,
    `        Self::from_val(&emlite::Val::take_ownership(v))`,
    `    }`,
    `    fn as_handle(&self) -> emlite::env::Handle {`,
    `        self.inner.as_handle()`,
    `    }`,
    `}`,
    `impl core::ops::Deref for ${dict.name} {`,
    `    type Target = emlite::Val;`,
    `    fn deref(&self) -> &Self::Target {`,
    `        &self.inner`,
    `    }`,
    `}`,
    `impl core::ops::DerefMut for ${dict.name} {`,
    `    fn deref_mut(&mut self) -> &mut Self::Target {`,
    `        &mut self.inner`,
    `    }`,
    `}`,
    `impl AsRef<emlite::Val> for ${dict.name} {`,
    `    fn as_ref(&self) -> &emlite::Val {`,
    `        &self.inner`,
    `    }`,
    `}`,
    `impl AsMut<emlite::Val> for ${dict.name} {`,
    `    fn as_mut(&mut self) -> &mut emlite::Val {`,
    `      &mut self.inner`,
    `  }`,
    `}`,
    `impl From<${dict.name}> for emlite::Val {`,
    `    fn from(s: ${dict.name}) -> emlite::Val {`,
    `        let handle = s.inner.as_handle();`,
    `        core::mem::forget(s);`,
    `        emlite::Val::take_ownership(handle)`,
    `    }`,
    `}`,
    `impl From<&${dict.name}> for emlite::Val {`,
    `    fn from(s: &${dict.name}) -> emlite::Val {`,
    `        s.inner.clone()`,
    `    }`,
    `}`,
    ""
  );
  dict.members.forEach((m) => {
    const S = emitAttr(m, dict.name);
    src.push(...S);
  });
}

function refNames(member) {
  const out = new Set();
  const scan = (t) => {
    if (!t) return;

    if (Array.isArray(t)) return t.forEach(scan);

    if (typeof t === "object" && t.generic && t.idlType) {
      if (Array.isArray(t.idlType)) t.idlType.forEach(scan);
      else scan(t.idlType);
    }

    const n = flat(t).n;
    if (
      /^(undefined|any|object|Promise|DOMString|USVString|ByteString|CSSOMString|boolean)$/.test(
        n
      ) ||
      /long|short|float|double/.test(n) ||
      n.startsWith("__")
    )
      return;

    out.add(n);

    if (typeof t === "object" && t.idlType) scan(t.idlType);
  };

  if (member.type === "attribute") scan(member.idlType);
  else if (member.type === "operation") {
    scan(member.idlType);
    member.arguments.forEach((a) => scan(a.idlType));
  }
  return out;
}

export function generate(specAst) {
  const interfaces = new Map();
  const mixins = new Map();
  const includeRel = [];
  const dicts = new Map();
  const namespaces = [];

  for (const ast of Object.values(specAst)) {
    for (const def of ast) {
      switch (def.type) {
        case "interface": {
          const rec = interfaces.get(def.name) || {
            partials: [],
            includes: [],
          };
          if (def.partial) rec.partials.push(def);
          else rec.base = def;
          interfaces.set(def.name, rec);
          break;
        }
        case "interface mixin":
          mixins.set(def.name, def);
          break;
        case "includes":
          includeRel.push({ target: def.target, mixin: def.includes });
          break;
        case "dictionary":
          dicts.set(def.name, def);
          break;
        case "enum":
          enums.set(def.name, def);
          break;
        case "namespace":
          namespaces.push(def);
          break;
        case "callback":
        case "callback interface":
          callbacks.add(def.name);
          break;
        case "typedef":
          typedefs.set(def.name, def.idlType);
          break;
      }
    }
  }

  {
    const src = ["\n"];
    for (const e of enums.values()) {
     src.push(
        `#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]`,
        `pub enum ${e.name} {`);
      for (const v of e.values) {
        src.push(
          `    ${fixIdent(v.value).toUpperCase()},`);
      }
      src.push("}");
      src.push(
      `impl FromVal for ${e.name} {`,
      `    fn from_val(v: &emlite::Val) -> Self {`,
      `         match v.as_::<&str>() {`
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
      `    fn take_ownership(v: emlite::env::Handle) -> Self {`,
      `        Self::from_val(&emlite::Val::take_ownership(v))`,
      `    }`,
      `    fn as_handle(&self) -> emlite::env::Handle {`,
      `        emlite::Val::from(*self).as_handle()`,
      `    }`,
      `}`,
      );
      src.push(
        `impl From<${e.name}> for emlite::Val {`,
        `    fn from(s: ${e.name}) -> emlite::Val {`,
        `         match s {`);
      for (const v of e.values) {
        src.push(
          `            ${e.name}::${fixIdent(v.value).toUpperCase()} => emlite::Val::from("${
            v.value
          }"),`
        );
      }
      src.push("         }");
      src.push(`    }`,
        `}`,
        `impl From<&${e.name}> for emlite::Val {`,
        `    fn from(s: &${e.name}) -> emlite::Val {`,
        `         match *s {`);
      for (const v of e.values) {
        src.push(
          `            ${e.name}::${fixIdent(v.value).toUpperCase()} => emlite::Val::from("${
            v.value
          }"),`
        );
      }
      src.push("         }");
      src.push(`    }`,
        `}`,
        ""
      );
      src.push("");
    }

    writeSrcFile("enums", src);
  }

  includeRel.forEach(({ target, mixin }) => {
    const rec = interfaces.get(target);
    if (rec) rec.includes.push(mixin);
  });

  for (const [name, rec] of interfaces) {
    const mem = new Map();
    const cons = new Map();
    const addM = (m) => mem.set(`${m.type}:${m.name}`, m);
    const addC = (c) => cons.set(c.name, c);

    if (rec.base) {
      rec.base.members.forEach(addM);
      rec.base.constants?.forEach(addC);
    }
    rec.partials.forEach((p) => {
      p.members.forEach(addM);
      p.constants?.forEach(addC);
    });
    rec.includes.forEach((mn) => {
      const mx = mixins.get(mn);
      if (mx) {
        mx.members.forEach(addM);
        mx.constants?.forEach(addC);
      }
    });

    rec.members = [...mem.values()];
    rec.consts = [...cons.values()];
    interfaces.set(name, rec);
  }

  for (const [iname, rec] of interfaces) {
    const src = ["\n"];
    let parent = rec.base?.inheritance || null;

    const fwd = new Set();
    const srcInc = new Set();
    const localEnums = new Set();
    const localDicts = [];

    rec.members.forEach((m) => {
      refNames(m).forEach((tn) => {
        if (dicts.has(tn)) {
          if (!dictOwner.has(tn)) {
            // not sure how to deal with these!
            fwd.add(tn);
            localDicts.push(dicts.get(tn));
          }
          return;
        }
        if (enums.has(tn)) {
          localEnums.add(enums.get(tn));
          return;
        }
        if (interfaces.has(tn) && tn !== parent) {
          // not sure how to deal with these!
          fwd.add(tn);
          srcInc.add(`${tn}.hpp`);
        }
      });
    });

    localDicts.forEach((d) => {
      d.members.forEach((dm) => {
        refNames(dm).forEach((tn) => {
          if (dicts.has(tn)) {
            return;
          }
          if (interfaces.has(tn) && tn !== parent) {
            fwd.add(tn);
            srcInc.add(`${tn}.hpp`);
          }
        });
      });
    });

    localDicts.forEach((d) => embedDict(d, src, iname));

    if (!parent) parent = "emlite::Val";

    src.push(
      `#[derive(Clone, Debug, PartialEq, PartialOrd)]`,
      `#[repr(transparent)]`,
      `pub struct ${iname} {`,
      `    inner: ${parent},`,
      `}`,
      `impl FromVal for ${iname} {`,
      `    fn from_val(v: &emlite::Val) -> Self {`,
      `        ${iname} { inner: ${parent}::from_val(v) }`,
      `    }`,
      `    fn take_ownership(v: emlite::env::Handle) -> Self {`,
      `        Self::from_val(&emlite::Val::take_ownership(v))`,
      `    }`,
      `    fn as_handle(&self) -> emlite::env::Handle {`,
      `        self.inner.as_handle()`,
      `    }`,
      `}`,
      `impl core::ops::Deref for ${iname} {`,
      `    type Target = ${parent};`,
      `    fn deref(&self) -> &Self::Target {`,
      `        &self.inner`,
      `    }`,
      `}`,
      `impl core::ops::DerefMut for ${iname} {`,
      `    fn deref_mut(&mut self) -> &mut Self::Target {`,
      `        &mut self.inner`,
      `    }`,
      `}`,
      `impl AsRef<emlite::Val> for ${iname} {`,
      `    fn as_ref(&self) -> &emlite::Val {`,
      `        &self.inner`,
      `    }`,
      `}`,
      `impl AsMut<emlite::Val> for ${iname} {`,
      `    fn as_mut(&mut self) -> &mut emlite::Val {`,
      `      &mut self.inner`,
      `  }`,
      `}`,
      `impl From<${iname}> for emlite::Val {`,
      `    fn from(s: ${iname}) -> emlite::Val {`,
      `        let handle = s.inner.as_handle();`,
      `        core::mem::forget(s);`,
      `        emlite::Val::take_ownership(handle)`,
      `    }`,
      `}`,
      `impl From<&${iname}> for emlite::Val {`,
      `    fn from(s: &${iname}) -> emlite::Val {`,
      `        s.inner.clone().into()`,
      `    }`,
      `}`,
      `jsbind::utils::impl_dyn_cast!(${iname});`,
      ""
    );
    src.push("");

    rec.consts.forEach((c) =>
      src.push(`    const ${c.name}: ${rust(c.idlType)} = ${c.value.value};`)
    );
    rec.members.forEach((m) => {
      const isStatic = m.static === true || m.special === "static";
      if (m.type === "attribute") {
        const S = emitAttr(m, iname, isStatic);
        src.push(...S);
      } else if (m.type === "operation") {
        const S = emitOp(m, iname, isStatic);
        src.push(...S);
      } else if (
        m.type === "constructor" ||
        (m.type === "operation" && m.special === "constructor")
      ) {
        const S = emitCtor(m, iname, parent);
        src.push(...S);
      }
    });

    writeSrcFile(iname, src);
  }
  for (const ns of namespaces) {
    const src = [];

    ns.members
      .filter((m) => m.type === "operation")
      .forEach((op) => {
        const ret = rust(op.idlType || "undefined");
        const cppName = fixIdent(op.name);
        const variants = variantsOf(op.arguments);
        let i = 0;
        const sz = variants.length;
        for (const v of variants) {
          const declSrc = argDecl(v);
          const callArgs = v
            .map((a) => `${fixIdent(a.name)}.into()`)
            .join(", ");

          const callExpr = `emlite::Val::global("${ns.name}").call("${
            op.name
          }", &[${callArgs ? callArgs + ", " : ""}])`;

          src.push(
            `pub fn ${cppName}${sz === 1 ? "" : i}(${declSrc}) -> ${ret} {`,
            `    ${callExpr}.as_::<${ret}>()`,
            `}`,
            ""
          );
          i += 1;
        }
      });
    writeSrcFile(ns.name, src);
  }
}
