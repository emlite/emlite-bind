use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSKeyframeRule {
    inner: CSSRule,
}
impl FromVal for CSSKeyframeRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSKeyframeRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSKeyframeRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSKeyframeRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSKeyframeRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSKeyframeRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSKeyframeRule> for emlite::Val {
    fn from(s: CSSKeyframeRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSKeyframeRule);


impl CSSKeyframeRule {
    pub fn key_text(&self) -> CSSOMString {
        self.inner.get("keyText").as_::<CSSOMString>()
    }

    pub fn set_key_text(&mut self, value: CSSOMString) {
        self.inner.set("keyText", value);
    }

}
impl CSSKeyframeRule {
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }

}
