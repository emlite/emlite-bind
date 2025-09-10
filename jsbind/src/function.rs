//! Notes on callback lifetime
//! Callbacks created via `Closure::bind*` or `Closure::new` are exposed to JS and
//! kept alive by handles in the runtime. There is no automatic disposal on
//! non–WASI‑p2 targets; callbacks live until the handle map is reset or all
//! references are dropped on the JS side. Prefer short‑lived closures when
//! appropriate, or add manual cleanup in your app if needed.
use crate::any::Any;
use crate::array::Array;
use crate::error::JsError;
use crate::utils::*;
use alloc::string::String;
use alloc::vec::Vec;
use emlite::FromVal;

/// Wrapper around a runtime JavaScript `Function` object.
///
/// Unlike [`Closure`], this is not created from a Rust callback; it
/// simply holds an existing function reference coming from JS.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Function {
    inner: emlite::Val,
}

bind!(Function);
impl_dyn_cast!(Function);

impl Function {
    /// Attempt to fetch `globalThis[name]` and treat it as a function.
    /// Returns `None` if the global is `undefined` or not callable.
    pub fn global(name: &str) -> Option<Self> {
        let v = emlite::Val::global(name);
        if v.is_function() {
            Some(v.as_::<Self>())
        } else {
            None
        }
    }

    /// Fetch `globalThis[name]` as a function or return a `JsError` if missing/not callable.
    pub fn try_global(name: &str) -> Result<Self, JsError> {
        let v = emlite::Val::global(name);
        if v.is_function() {
            Ok(v.as_::<Self>())
        } else {
            Err(JsError::new("global is not a function"))
        }
    }

    /// Build a new function via the JS `Function` constructor:
    /// `new Function(arg1, arg2, ..., body)`.
    ///
    /// ```
    /// let f = Function::new(&["a", "b"], "return a + b;").unwrap();
    /// let sum: i32 = f.call(&Any::undefined(), &[1.into(), 2.into()]).unwrap().as_();
    /// assert_eq!(sum, 3);
    /// ```
    pub fn new<S: AsRef<str>>(args: &[S], body: &str) -> Result<Self, JsError> {
        let ctor = emlite::Val::global("Function");
        let mut a: Vec<emlite::Val> = args.iter().map(|s| s.as_ref().into()).collect();
        a.push(body.into());
        ctor.new(&a).as_::<Result<Self, JsError>>()
    }
}

impl Function {
    /// Call `fn.call(this, …args)`.  
    /// Returns the raw JS value so the caller can choose the concrete type.
    pub fn call(&self, this_arg: &Any, args: &[Any]) -> Result<Any, JsError> {
        // prepend `this` then use `Function.prototype.call`
        let mut v: Vec<emlite::Val> = Vec::with_capacity(args.len() + 1);
        v.push(this_arg.clone());
        v.extend(args.iter().cloned());
        self.inner.call("call", &v).as_()
    }

    /// Call `fn.apply(this, args_array)`.
    pub fn apply(&self, this_arg: &Any, args_array: &Array) -> Result<Any, JsError> {
        self.inner
            .call("apply", &[this_arg.clone(), args_array.clone().into()])
            .as_()
    }

    /// Bind a new `this` argument (`fn.bind(this, …pre_args)`).
    pub fn bind(&self, this_arg: &Any, pre_args: &[Any]) -> Result<Self, JsError> {
        let mut a: Vec<emlite::Val> = Vec::with_capacity(pre_args.len() + 1);
        a.push(this_arg.clone());
        a.extend(pre_args.iter().cloned());
        self.inner.call("bind", &a).as_::<Result<Self, JsError>>()
    }
}

impl From<Closure> for Function {
    fn from(c: Closure) -> Self {
        c.as_::<Self>()
    }
}

impl core::fmt::Display for Function {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // JS › fn.toString()
        let s: Option<String> = self.inner.call("toString", &[]).as_();
        if let Some(s) = s {
            f.write_str(&s)
        } else {
            f.write_str("unnamed")
        }
    }
}

impl From<&Closure> for Function {
    fn from(c: &Closure) -> Self {
        c.clone().into()
    }
}
impl From<&Function> for Closure {
    fn from(f: &Function) -> Self {
        f.clone().as_()
    }
}

/// The inner value is guaranteed at runtime to be callable (`typeof v ===
/// "function"`)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Closure {
    inner: emlite::Val,
}

bind!(Closure);

impl Closure {
    /// Build a `Closure` from a raw closure that receives the argument
    /// array as a slice of `emlite::Val` and must itself return an
    /// `emlite::Val`.
    ///
    /// Use this when the callback signature is not known until runtime or when
    /// you want to support variadic JavaScript calls.
    pub fn new<F>(cb: F) -> Closure
    where
        F: FnMut(&[emlite::Val]) -> emlite::Val,
    {
        Closure {
            inner: emlite::Val::make_fn(cb),
        }
    }

    /// Create a JS function with no parameters whose Rust body returns a
    /// value convertible into `emlite::Val`.
    pub fn bind0<Ret, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        F: FnMut() -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |_| {
                let r: Ret = cb();
                r.into()
            }),
        }
    }

    /// Create a typed JS function with one parameter.
    ///
    /// `Arg1` – Rust type the first JS argument should be converted into.
    /// `Ret`  – type converted from Rust back to JS for the return value.
    pub fn bind1<Ret, Arg1, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        F: FnMut(Arg1) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let arg1 = args[0].as_::<Arg1>();
                let r: Ret = cb(arg1);
                r.into()
            }),
        }
    }

    /// Two‑argument typed JS function.  See [`bind1`] for the generic
    /// parameter meanings.
    pub fn bind2<Ret, Arg1, Arg2, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        F: FnMut(Arg1, Arg2) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(args[0].as_::<Arg1>(), args[1].as_::<Arg2>());
                r.into()
            }),
        }
    }

    /// Three‑argument typed JS function.
    pub fn bind3<Ret, Arg1, Arg2, Arg3, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        F: FnMut(Arg1, Arg2, Arg3) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(
                    args[0].as_::<Arg1>(),
                    args[1].as_::<Arg2>(),
                    args[2].as_::<Arg3>(),
                );
                r.into()
            }),
        }
    }

    /// Four‑argument typed JS function.
    pub fn bind4<Ret, Arg1, Arg2, Arg3, Arg4, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(
                    args[0].as_::<Arg1>(),
                    args[1].as_::<Arg2>(),
                    args[2].as_::<Arg3>(),
                    args[3].as_::<Arg4>(),
                );
                r.into()
            }),
        }
    }

    /// Five‑argument typed JS function.
    pub fn bind5<Ret, Arg1, Arg2, Arg3, Arg4, Arg5, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(
                    args[0].as_::<Arg1>(),
                    args[1].as_::<Arg2>(),
                    args[2].as_::<Arg3>(),
                    args[3].as_::<Arg4>(),
                    args[4].as_::<Arg5>(),
                );
                r.into()
            }),
        }
    }

    /// Six‑argument typed JS function.
    pub fn bind6<Ret, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        Arg6: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(
                    args[0].as_::<Arg1>(),
                    args[1].as_::<Arg2>(),
                    args[2].as_::<Arg3>(),
                    args[3].as_::<Arg4>(),
                    args[4].as_::<Arg5>(),
                    args[5].as_::<Arg6>(),
                );
                r.into()
            }),
        }
    }

    /// Seven‑argument typed JS function.
    pub fn bind7<Ret, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, F>(mut cb: F) -> Closure
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        Arg6: FromVal,
        Arg7: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7) -> Ret,
    {
        Closure {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(
                    args[0].as_::<Arg1>(),
                    args[1].as_::<Arg2>(),
                    args[2].as_::<Arg3>(),
                    args[3].as_::<Arg4>(),
                    args[4].as_::<Arg5>(),
                    args[5].as_::<Arg6>(),
                    args[6].as_::<Arg7>(),
                );
                r.into()
            }),
        }
    }
}
