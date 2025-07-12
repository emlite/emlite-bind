use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for CSSParserFunction {
    type Target = CSSParserValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSParserFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSParserFunction> for emlite::Val {
    fn from(s: CSSParserFunction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSParserFunction {
    pub fn new(
        name: jsbind::DOMString,
        args: jsbind::Sequence<jsbind::Sequence<CSSParserValue>>,
    ) -> CSSParserFunction {
        Self {
            inner: emlite::Val::global("CSSParserFunction")
                .new(&[name.into(), args.into()])
                .as_::<CSSParserValue>(),
        }
    }
}
impl CSSParserFunction {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl CSSParserFunction {
    pub fn args(&self) -> jsbind::FrozenArray<jsbind::FrozenArray<CSSParserValue>> {
        self.inner
            .get("args")
            .as_::<jsbind::FrozenArray<jsbind::FrozenArray<CSSParserValue>>>()
    }
}
