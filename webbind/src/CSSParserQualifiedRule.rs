use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSParserQualifiedRule {
    inner: CSSParserRule,
}
impl FromVal for CSSParserQualifiedRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserQualifiedRule {
            inner: CSSParserRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserQualifiedRule {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserQualifiedRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSParserQualifiedRule> for emlite::Val {
    fn from(s: CSSParserQualifiedRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSParserQualifiedRule {
    pub fn new0(prelude: jsbind::Sequence<jsbind::Any>) -> CSSParserQualifiedRule {
        Self {
            inner: emlite::Val::global("CSSParserQualifiedRule")
                .new(&[prelude.into()])
                .as_::<CSSParserRule>(),
        }
    }

    pub fn new1(
        prelude: jsbind::Sequence<jsbind::Any>,
        body: jsbind::Sequence<CSSParserRule>,
    ) -> CSSParserQualifiedRule {
        Self {
            inner: emlite::Val::global("CSSParserQualifiedRule")
                .new(&[prelude.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserQualifiedRule {
    pub fn prelude(&self) -> jsbind::FrozenArray<CSSParserValue> {
        self.inner
            .get("prelude")
            .as_::<jsbind::FrozenArray<CSSParserValue>>()
    }
}
impl CSSParserQualifiedRule {
    pub fn body(&self) -> jsbind::FrozenArray<CSSParserRule> {
        self.inner
            .get("body")
            .as_::<jsbind::FrozenArray<CSSParserRule>>()
    }
}
