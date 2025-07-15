use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleProperties {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSStyleProperties {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStyleProperties { inner: CSSStyleDeclaration::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleProperties {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSStyleProperties {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSStyleProperties {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSStyleProperties> for emlite::Val {
    fn from(s: CSSStyleProperties) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleProperties);


impl CSSStyleProperties {
    pub fn css_float(&self) -> CSSOMString {
        self.inner.get("cssFloat").as_::<CSSOMString>()
    }

    pub fn set_css_float(&mut self, value: CSSOMString) {
        self.inner.set("cssFloat", value);
    }

}
