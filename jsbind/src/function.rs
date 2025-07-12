use crate::utils::bind;
use emlite::FromVal;

#[derive(Clone, Debug)]
pub struct Function {
    inner: emlite::Val,
}

bind!(Function);

impl Function {
    pub fn new<F: FnMut(&[emlite::Val]) -> emlite::Val>(cb: F) -> Function {
        Function {
            inner: emlite::Val::make_fn(cb),
        }
    }

    pub fn typed0<Ret: Into<emlite::Val>, F: FnMut() -> Ret>(mut cb: F) -> Function {
        Function {
            inner: emlite::Val::make_fn(move |_| {
                let r: Ret = cb();
                r.into()
            }),
        }
    }

    pub fn typed1<Ret: Into<emlite::Val>, Arg1: FromVal, F: FnMut(Arg1) -> Ret>(
        mut cb: F,
    ) -> Function {
        Function {
            inner: emlite::Val::make_fn(move |args| {
                let arg1 = args[0].as_::<Arg1>();
                let r: Ret = cb(arg1);
                r.into()
            }),
        }
    }

    pub fn typed2<
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        F: FnMut(Arg1, Arg2) -> Ret,
    >(
        mut cb: F,
    ) -> Function {
        Function {
            inner: emlite::Val::make_fn(move |args| {
                let r: Ret = cb(args[0].as_::<Arg1>(), args[1].as_::<Arg2>());
                r.into()
            }),
        }
    }

    pub fn typed3<
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        F: FnMut(Arg1, Arg2, Arg3) -> Ret,
    >(
        mut cb: F,
    ) -> Function {
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

    pub fn typed4<
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4) -> Ret,
    >(
        mut cb: F,
    ) -> Function {
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

    pub fn typed5<
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5) -> Ret,
    >(
        mut cb: F,
    ) -> Function {
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

    pub fn typed6<
        Ret: Into<emlite::Val>,
        Arg1: FromVal,
        Arg2: FromVal,
        Arg3: FromVal,
        Arg4: FromVal,
        Arg5: FromVal,
        Arg6: FromVal,
        F: FnMut(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6) -> Ret,
    >(
        mut cb: F,
    ) -> Function {
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
