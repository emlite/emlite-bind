use super::*;

#[derive(Clone, Debug)]
pub struct CSSPageRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSPageRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPageRule {
            inner: CSSGroupingRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSPageRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSPageRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPageRule> for emlite::Val {
    fn from(s: CSSPageRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPageRule {
    pub fn selector_text(&self) -> jsbind::CSSOMString {
        self.inner.get("selectorText").as_::<jsbind::CSSOMString>()
    }

    pub fn set_selector_text(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("selectorText", value);
    }
}
impl CSSPageRule {
    pub fn style(&self) -> CSSPageDescriptors {
        self.inner.get("style").as_::<CSSPageDescriptors>()
    }
}
