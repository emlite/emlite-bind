use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSMarginRule {
    inner: CSSRule,
}
impl FromVal for CSSMarginRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMarginRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMarginRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMarginRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMarginRule> for emlite::Val {
    fn from(s: CSSMarginRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMarginRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl CSSMarginRule {
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
