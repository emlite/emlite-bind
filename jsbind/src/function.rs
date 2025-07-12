use crate::utils::bind;
use emlite::FromVal;

/// The inner value is guaranteed at runtime to be callable (`typeof v ===
/// "function"`).  All methods are zero‑cost delegates to `emlite::Val` helpers.
#[derive(Clone, Debug)]
pub struct Function {
    /// Underlying JavaScript function object.
    inner: emlite::Val,
}

bind!(Function);

impl Function {
    /// Build a `Function` from a raw closure that receives the argument
    /// array as a slice of `emlite::Val` and must itself return an
    /// `emlite::Val`.
    ///
    /// Use this when the callback signature is not known until runtime or when
    /// you want to support variadic JavaScript calls.
    pub fn new<F>(cb: F) -> Function
    where
        F: FnMut(&[emlite::Val]) -> emlite::Val,
    {
        Function {
            inner: emlite::Val::make_fn(cb),
        }
    }

    /// Create a JS function with no parameters whose Rust body returns a
    /// value convertible into `emlite::Val`.
    pub fn typed0<Ret, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        F: FnMut() -> Ret,
    {
        Function {
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
    pub fn typed1<Ret, Arg1, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        F: FnMut(Arg1) -> Ret,
    {
        Function {
            inner: emlite::Val::make_fn(move |args| {
                let arg1 = args[0].as_::<Arg1>();
                let r: Ret = cb(arg1);
                r.into()
            }),
        }
    }

    /// Two‑argument typed JS function.  See [`typed1`] for the generic
    /// parameter meanings.
    pub fn typed2<Ret, Arg1, Arg2, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        F: FnMut(Arg1, Arg2) -> Ret,
    {
        Function {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(args[0].as_::<Arg1>(), args[1].as_::<Arg2>());
                r.into()
            }),
        }
    }

    /// Three‑argument typed JS function.
    pub fn typed3<Ret, Arg1, Arg2, Arg3, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        F: FnMut(Arg1, Arg2, Arg3) -> Ret,
    {
        Function {
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
    pub fn typed4<Ret, Arg1, Arg2, Arg3, Arg4, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4) -> Ret,
    {
        Function {
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
    pub fn typed5<Ret, Arg1, Arg2, Arg3, Arg4, Arg5, F>(mut cb: F) -> Function
    where
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5) -> Ret,
    {
        Function {
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
    pub fn typed6<Ret, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, F>(mut cb: F) -> Function
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
        Function {
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
}
