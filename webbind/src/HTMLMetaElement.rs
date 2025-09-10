use super::*;

/// The HTMLMetaElement class.
/// [`HTMLMetaElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMetaElement {
    inner: HTMLElement,
}

impl FromVal for HTMLMetaElement {
    fn from_val(v: &Any) -> Self {
        HTMLMetaElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLMetaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLMetaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLMetaElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLMetaElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLMetaElement> for Any {
    fn from(s: HTMLMetaElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLMetaElement> for Any {
    fn from(s: &HTMLMetaElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLMetaElement);

impl HTMLMetaElement {
    /// The `new HTMLMetaElement(..)` constructor, creating a new HTMLMetaElement instance
    pub fn new() -> HTMLMetaElement {
        Self {
            inner: Any::global("HTMLMetaElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLMetaElement {
    /// Getter of the `name` attribute.
    /// [`HTMLMetaElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLMetaElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLMetaElement {
    /// Getter of the `httpEquiv` attribute.
    /// [`HTMLMetaElement.httpEquiv`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)
    pub fn http_equiv(&self) -> JsString {
        self.inner.get("httpEquiv").as_::<JsString>()
    }

    /// Setter of the `httpEquiv` attribute.
    /// [`HTMLMetaElement.httpEquiv`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)
    pub fn set_http_equiv(&mut self, value: &JsString) {
        self.inner.set("httpEquiv", value);
    }
}
impl HTMLMetaElement {
    /// Getter of the `content` attribute.
    /// [`HTMLMetaElement.content`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)
    pub fn content(&self) -> JsString {
        self.inner.get("content").as_::<JsString>()
    }

    /// Setter of the `content` attribute.
    /// [`HTMLMetaElement.content`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)
    pub fn set_content(&mut self, value: &JsString) {
        self.inner.set("content", value);
    }
}
impl HTMLMetaElement {
    /// Getter of the `media` attribute.
    /// [`HTMLMetaElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    /// [`HTMLMetaElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/media)
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl HTMLMetaElement {
    /// Getter of the `scheme` attribute.
    /// [`HTMLMetaElement.scheme`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)
    pub fn scheme(&self) -> JsString {
        self.inner.get("scheme").as_::<JsString>()
    }

    /// Setter of the `scheme` attribute.
    /// [`HTMLMetaElement.scheme`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)
    pub fn set_scheme(&mut self, value: &JsString) {
        self.inner.set("scheme", value);
    }
}
