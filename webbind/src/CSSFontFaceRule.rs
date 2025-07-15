use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFaceRule {
    inner: CSSRule,
}
impl FromVal for CSSFontFaceRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontFaceRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFontFaceRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFontFaceRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSFontFaceRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSFontFaceRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSFontFaceRule> for emlite::Val {
    fn from(s: CSSFontFaceRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSFontFaceRule);


impl CSSFontFaceRule {
    pub fn style(&self) -> CSSFontFaceDescriptors {
        self.inner.get("style").as_::<CSSFontFaceDescriptors>()
    }

}
