use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSStyleRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStyleRule { inner: CSSGroupingRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSStyleRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSStyleRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSStyleRule> for emlite::Val {
    fn from(s: CSSStyleRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleRule);


impl CSSStyleRule {
    pub fn selector_text(&self) -> CSSOMString {
        self.inner.get("selectorText").as_::<CSSOMString>()
    }

    pub fn set_selector_text(&mut self, value: CSSOMString) {
        self.inner.set("selectorText", value);
    }

}
impl CSSStyleRule {
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }

}
impl CSSStyleRule {
    pub fn style_map(&self) -> StylePropertyMap {
        self.inner.get("styleMap").as_::<StylePropertyMap>()
    }

}
