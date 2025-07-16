use super::*;

/// The CSSImportRule class.
/// [`CSSImportRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSImportRule {
    inner: CSSRule,
}
impl FromVal for CSSImportRule {
    fn from_val(v: &Any) -> Self {
        CSSImportRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CSSImportRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSImportRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSImportRule> for Any {
    fn from(s: CSSImportRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSImportRule> for Any {
    fn from(s: &CSSImportRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSImportRule);

impl CSSImportRule {
    /// Getter of the `href` attribute.
    /// [`CSSImportRule.href`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)
    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }
}
impl CSSImportRule {
    /// Getter of the `media` attribute.
    /// [`CSSImportRule.media`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }
}
impl CSSImportRule {
    /// Getter of the `styleSheet` attribute.
    /// [`CSSImportRule.styleSheet`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)
    pub fn style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("styleSheet").as_::<CSSStyleSheet>()
    }
}
impl CSSImportRule {
    /// Getter of the `layerName` attribute.
    /// [`CSSImportRule.layerName`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/layerName)
    pub fn layer_name(&self) -> String {
        self.inner.get("layerName").as_::<String>()
    }
}
impl CSSImportRule {
    /// Getter of the `supportsText` attribute.
    /// [`CSSImportRule.supportsText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/supportsText)
    pub fn supports_text(&self) -> String {
        self.inner.get("supportsText").as_::<String>()
    }
}
