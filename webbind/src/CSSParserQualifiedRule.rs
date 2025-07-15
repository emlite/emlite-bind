use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for CSSParserQualifiedRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSParserQualifiedRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&CSSParserQualifiedRule> for emlite::Val {
    fn from(s: &CSSParserQualifiedRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserQualifiedRule);

impl CSSParserQualifiedRule {
    pub fn new0(prelude: &Sequence<Any>) -> CSSParserQualifiedRule {
        Self {
            inner: emlite::Val::global("CSSParserQualifiedRule")
                .new(&[prelude.into()])
                .as_::<CSSParserRule>(),
        }
    }

    pub fn new1(prelude: &Sequence<Any>, body: &Sequence<CSSParserRule>) -> CSSParserQualifiedRule {
        Self {
            inner: emlite::Val::global("CSSParserQualifiedRule")
                .new(&[prelude.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserQualifiedRule {
    pub fn prelude(&self) -> FrozenArray<CSSParserValue> {
        self.inner
            .get("prelude")
            .as_::<FrozenArray<CSSParserValue>>()
    }
}
impl CSSParserQualifiedRule {
    pub fn body(&self) -> FrozenArray<CSSParserRule> {
        self.inner.get("body").as_::<FrozenArray<CSSParserRule>>()
    }
}
