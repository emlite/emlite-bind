use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSImportRule {
    inner: CSSRule,
}
impl FromVal for CSSImportRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSImportRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSImportRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSImportRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSImportRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSImportRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSImportRule> for emlite::Val {
    fn from(s: CSSImportRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSImportRule);


impl CSSImportRule {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

}
impl CSSImportRule {
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }

}
impl CSSImportRule {
    pub fn style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("styleSheet").as_::<CSSStyleSheet>()
    }

}
impl CSSImportRule {
    pub fn layer_name(&self) -> CSSOMString {
        self.inner.get("layerName").as_::<CSSOMString>()
    }

}
impl CSSImportRule {
    pub fn supports_text(&self) -> CSSOMString {
        self.inner.get("supportsText").as_::<CSSOMString>()
    }

}
