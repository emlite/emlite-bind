use super::*;

/// The StyleSheet class.
/// [`StyleSheet`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StyleSheet {
    inner: Any,
}
impl FromVal for StyleSheet {
    fn from_val(v: &Any) -> Self {
        StyleSheet {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StyleSheet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StyleSheet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StyleSheet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StyleSheet> for Any {
    fn from(s: StyleSheet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StyleSheet> for Any {
    fn from(s: &StyleSheet) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StyleSheet);

impl StyleSheet {
    /// Getter of the `type` attribute.
    /// [`StyleSheet.type`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)
    pub fn type_(&self) -> CSSOMString {
        self.inner.get("type").as_::<CSSOMString>()
    }
}
impl StyleSheet {
    /// Getter of the `href` attribute.
    /// [`StyleSheet.href`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }
}
impl StyleSheet {
    /// Getter of the `ownerNode` attribute.
    /// [`StyleSheet.ownerNode`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)
    pub fn owner_node(&self) -> Any {
        self.inner.get("ownerNode").as_::<Any>()
    }
}
impl StyleSheet {
    /// Getter of the `parentStyleSheet` attribute.
    /// [`StyleSheet.parentStyleSheet`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)
    pub fn parent_style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("parentStyleSheet").as_::<CSSStyleSheet>()
    }
}
impl StyleSheet {
    /// Getter of the `title` attribute.
    /// [`StyleSheet.title`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }
}
impl StyleSheet {
    /// Getter of the `media` attribute.
    /// [`StyleSheet.media`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }
}
impl StyleSheet {
    /// Getter of the `disabled` attribute.
    /// [`StyleSheet.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`StyleSheet.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
