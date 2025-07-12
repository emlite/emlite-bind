use super::*;

#[derive(Clone, Debug)]
pub struct CSSImportRule {
    inner: CSSRule,
}
impl FromVal for CSSImportRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSImportRule {
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
impl std::ops::Deref for CSSImportRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSImportRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSImportRule> for emlite::Val {
    fn from(s: CSSImportRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSImportRule {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
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
    pub fn layer_name(&self) -> jsbind::CSSOMString {
        self.inner.get("layerName").as_::<jsbind::CSSOMString>()
    }
}
impl CSSImportRule {
    pub fn supports_text(&self) -> jsbind::CSSOMString {
        self.inner.get("supportsText").as_::<jsbind::CSSOMString>()
    }
}
