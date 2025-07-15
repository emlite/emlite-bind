use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserFunction {
    inner: CSSParserValue,
}
impl FromVal for CSSParserFunction {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserFunction {
            inner: CSSParserValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserFunction {
    type Target = CSSParserValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSParserFunction {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSParserFunction {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSParserFunction> for emlite::Val {
    fn from(s: CSSParserFunction) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSParserFunction> for emlite::Val {
    fn from(s: &CSSParserFunction) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserFunction);

impl CSSParserFunction {
    pub fn new(name: &str, args: &Sequence<Sequence<CSSParserValue>>) -> CSSParserFunction {
        Self {
            inner: emlite::Val::global("CSSParserFunction")
                .new(&[name.into(), args.into()])
                .as_::<CSSParserValue>(),
        }
    }
}
impl CSSParserFunction {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSParserFunction {
    pub fn args(&self) -> FrozenArray<FrozenArray<CSSParserValue>> {
        self.inner
            .get("args")
            .as_::<FrozenArray<FrozenArray<CSSParserValue>>>()
    }
}
