use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserAtRule {
    inner: CSSParserRule,
}
impl FromVal for CSSParserAtRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserAtRule {
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
impl core::ops::Deref for CSSParserAtRule {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserAtRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSParserAtRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSParserAtRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSParserAtRule> for emlite::Val {
    fn from(s: CSSParserAtRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserAtRule);

impl CSSParserAtRule {
    pub fn new0(
        name: jsbind::DOMString,
        prelude: jsbind::Sequence<jsbind::Any>,
    ) -> CSSParserAtRule {
        Self {
            inner: emlite::Val::global("CSSParserAtRule")
                .new(&[name.into(), prelude.into()])
                .as_::<CSSParserRule>(),
        }
    }

    pub fn new1(
        name: jsbind::DOMString,
        prelude: jsbind::Sequence<jsbind::Any>,
        body: jsbind::Sequence<CSSParserRule>,
    ) -> CSSParserAtRule {
        Self {
            inner: emlite::Val::global("CSSParserAtRule")
                .new(&[name.into(), prelude.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserAtRule {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl CSSParserAtRule {
    pub fn prelude(&self) -> jsbind::FrozenArray<CSSParserValue> {
        self.inner
            .get("prelude")
            .as_::<jsbind::FrozenArray<CSSParserValue>>()
    }
}
impl CSSParserAtRule {
    pub fn body(&self) -> jsbind::FrozenArray<CSSParserRule> {
        self.inner
            .get("body")
            .as_::<jsbind::FrozenArray<CSSParserRule>>()
    }
}
