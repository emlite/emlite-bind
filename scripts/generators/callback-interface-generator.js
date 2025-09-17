import { writeSrcFile, rust, fixIdent, argDecl } from "../utils.js";

// Emit a thin wrapper type for WebIDL `callback interface` definitions (Rust)
// The wrapper holds an `Any` and dispatches operations either to:
// - the underlying function value (when the wrapped value is a Function), or
// - the named operation on the object (when the wrapped value is an object with that method)
export function generateCallbackInterface(name, def /*, dependencies */) {
  const src = [""]; // writeSrcFile adds `use super::*;` automatically

  // Struct
  src.push(
    `/// Callback interface ${name}`,
    `/// Generated wrapper for WebIDL \`callback interface ${name}\``,
    `#[derive(Clone, Debug, PartialEq, PartialOrd)]`,
    `#[repr(transparent)]`,
    `pub struct ${name} {`,
    `    inner: Any,`,
    `}`,
    ""
  );

  // FromVal impl
  src.push(
    `impl FromVal for ${name} {`,
    `    fn from_val(v: &Any) -> Self {`,
    `        ${name} { inner: v.clone() }`,
    `    }`,
    `    fn take_ownership(v: AnyHandle) -> Self {`,
    `        ${name} { inner: Any::take_ownership(v) }`,
    `    }`,
    `    fn as_handle(&self) -> AnyHandle {`,
    `        self.inner.as_handle()`,
    `    }`,
    `}`,
    ""
  );

  // Deref to Any
  src.push(
    `impl core::ops::Deref for ${name} {`,
    `    type Target = Any;`,
    `    fn deref(&self) -> &Self::Target {`,
    `        &self.inner`,
    `    }`,
    `}`,
    "",
    `impl core::ops::DerefMut for ${name} {`,
    `    fn deref_mut(&mut self) -> &mut Self::Target {`,
    `        &mut self.inner`,
    `    }`,
    `}`,
    ""
  );

  // From <-> Any
  src.push(
    `impl From<${name}> for Any {`,
    `    fn from(s: ${name}) -> Any {`,
    `        let handle = s.inner.as_handle();`,
    `        core::mem::forget(s);`,
    `        Any::take_ownership(handle)`,
    `    }`,
    `}`,
    "",
    `impl From<&${name}> for Any {`,
    `    fn from(s: &${name}) -> Any {`,
    `        s.inner.clone().into()`,
    `    }`,
    `}`,
    ""
  );

  // Converting constructors
  src.push(
    `impl ${name} {`,
    `    /// Wrap a JavaScript function as a ${name}`,
    `    pub fn from_function(f: &Function) -> ${name} {`,
    `        ${name} { inner: Any::from(f.clone()) }`,
    `    }`,
    `}`,
    ""
  );

  // For each operation, add a typed from_closure helper and the dispatching method
  const ops = (def.members || []).filter((m) => m.type === "operation");
  ops.forEach((op) => {
    const ret = rust(op.idlType || "undefined");
    const args = op.arguments || [];
    const rustArgDecl = argDecl(args);
    const argNames = args.map((a) => fixIdent(a.name)).join(", ");
    const rustArgTypes = (args.length === 0)
      ? ""
      : args.map((a) => rust(a.idlType)).join(", ");

    // from_closure helper (typed)
    if (args.length <= 4) {
      // Use available Closure::bind{0..4} helpers depending on arity
      src.push(
        `impl ${name} {`,
        `    /// Build a ${name} from a typed Rust closure matching \`${op.name}\``,
        `    pub fn from_closure<F>(mut cb: F) -> ${name}`,
        `    where`,
        `        F: FnMut(${rustArgTypes}) -> ${ret} + 'static,`,
        `    {`,
        args.length === 0
          ? `        let c = Closure::bind0(move || cb());`
          : args.length === 1
          ? `        let c = Closure::bind1(move |a1: ${rust(args[0].idlType)}| cb(a1));`
          : args.length === 2
          ? `        let c = Closure::bind2(move |a1: ${rust(args[0].idlType)}, a2: ${rust(args[1].idlType)}| cb(a1, a2));`
          : args.length === 3
          ? `        let c = Closure::bind3(move |a1: ${rust(args[0].idlType)}, a2: ${rust(args[1].idlType)}, a3: ${rust(args[2].idlType)}| cb(a1, a2, a3));`
          : `        let c = Closure::bind4(move |a1: ${rust(args[0].idlType)}, a2: ${rust(args[1].idlType)}, a3: ${rust(args[2].idlType)}, a4: ${rust(args[3].idlType)}| cb(a1, a2, a3, a4));`,
        `        ${name}::from_function(&Function::from(&c))`,
        `    }`,
        `}`,
        ""
      );
    }

    // Dispatching method
    src.push(
      `impl ${name} {`,
      `    pub fn ${fixIdent(op.name)}(&self${rustArgDecl ? ", " + rustArgDecl : ""}) -> ${ret} {`,
      `        if self.inner.is_function() {`,
      `            // Call as a bare function`,
      `            self.inner.invoke(&[${args.map((a)=>`${fixIdent(a.name)}.into()`).join(", ")}]).as_::<${ret}>()`,
      `        } else {`,
      `            // Call the named method on the object`,
      `            self.inner.call("${op.name}", &[${args.map((a)=>`${fixIdent(a.name)}.into()`).join(", ")}]).as_::<${ret}>()`,
      `        }`,
      `    }`,
      `}`,
      ""
    );
  });

  writeSrcFile(name, src);
}

