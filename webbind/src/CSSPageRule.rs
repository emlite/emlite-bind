use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CSSPageRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPageRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSPageRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSPageRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSPageRule> for emlite::Val {
    fn from(s: CSSPageRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSPageRule> for emlite::Val {
    fn from(s: &CSSPageRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPageRule);

impl CSSPageRule {
    pub fn selector_text(&self) -> CSSOMString {
        self.inner.get("selectorText").as_::<CSSOMString>()
    }

    pub fn set_selector_text(&mut self, value: CSSOMString) {
        self.inner.set("selectorText", value);
    }
}
impl CSSPageRule {
    pub fn style(&self) -> CSSPageDescriptors {
        self.inner.get("style").as_::<CSSPageDescriptors>()
    }
}
