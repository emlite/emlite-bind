use super::*;

/// The SVGStyleElement class.
/// [`SVGStyleElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGStyleElement {
    inner: SVGElement,
}
impl FromVal for SVGStyleElement {
    fn from_val(v: &Any) -> Self {
        SVGStyleElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGStyleElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGStyleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGStyleElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGStyleElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGStyleElement> for Any {
    fn from(s: SVGStyleElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGStyleElement> for Any {
    fn from(s: &SVGStyleElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGStyleElement);

impl SVGStyleElement {
    /// Getter of the `type` attribute.
    /// [`SVGStyleElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`SVGStyleElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl SVGStyleElement {
    /// Getter of the `media` attribute.
    /// [`SVGStyleElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    /// [`SVGStyleElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl SVGStyleElement {
    /// Getter of the `title` attribute.
    /// [`SVGStyleElement.title`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    /// [`SVGStyleElement.title`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl SVGStyleElement {
    /// Getter of the `sheet` attribute.
    /// [`SVGStyleElement.sheet`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
