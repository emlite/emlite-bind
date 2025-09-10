use super::*;

/// The HTMLFrameElement class.
/// [`HTMLFrameElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFrameElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFrameElement {
    fn from_val(v: &Any) -> Self {
        HTMLFrameElement {
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

impl core::ops::Deref for HTMLFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFrameElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFrameElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLFrameElement> for Any {
    fn from(s: HTMLFrameElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFrameElement> for Any {
    fn from(s: &HTMLFrameElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFrameElement);

impl HTMLFrameElement {
    /// The `new HTMLFrameElement(..)` constructor, creating a new HTMLFrameElement instance
    pub fn new() -> HTMLFrameElement {
        Self {
            inner: Any::global("HTMLFrameElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFrameElement {
    /// Getter of the `name` attribute.
    /// [`HTMLFrameElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLFrameElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `scrolling` attribute.
    /// [`HTMLFrameElement.scrolling`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)
    pub fn scrolling(&self) -> JsString {
        self.inner.get("scrolling").as_::<JsString>()
    }

    /// Setter of the `scrolling` attribute.
    /// [`HTMLFrameElement.scrolling`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)
    pub fn set_scrolling(&mut self, value: &JsString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `src` attribute.
    /// [`HTMLFrameElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLFrameElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `frameBorder` attribute.
    /// [`HTMLFrameElement.frameBorder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)
    pub fn frame_border(&self) -> JsString {
        self.inner.get("frameBorder").as_::<JsString>()
    }

    /// Setter of the `frameBorder` attribute.
    /// [`HTMLFrameElement.frameBorder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)
    pub fn set_frame_border(&mut self, value: &JsString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `longDesc` attribute.
    /// [`HTMLFrameElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)
    pub fn long_desc(&self) -> JsString {
        self.inner.get("longDesc").as_::<JsString>()
    }

    /// Setter of the `longDesc` attribute.
    /// [`HTMLFrameElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)
    pub fn set_long_desc(&mut self, value: &JsString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `noResize` attribute.
    /// [`HTMLFrameElement.noResize`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)
    pub fn no_resize(&self) -> bool {
        self.inner.get("noResize").as_::<bool>()
    }

    /// Setter of the `noResize` attribute.
    /// [`HTMLFrameElement.noResize`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)
    pub fn set_no_resize(&mut self, value: bool) {
        self.inner.set("noResize", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `contentDocument` attribute.
    /// [`HTMLFrameElement.contentDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentDocument)
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }
}
impl HTMLFrameElement {
    /// Getter of the `contentWindow` attribute.
    /// [`HTMLFrameElement.contentWindow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentWindow)
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
    }
}
impl HTMLFrameElement {
    /// Getter of the `marginHeight` attribute.
    /// [`HTMLFrameElement.marginHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)
    pub fn margin_height(&self) -> JsString {
        self.inner.get("marginHeight").as_::<JsString>()
    }

    /// Setter of the `marginHeight` attribute.
    /// [`HTMLFrameElement.marginHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)
    pub fn set_margin_height(&mut self, value: &JsString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLFrameElement {
    /// Getter of the `marginWidth` attribute.
    /// [`HTMLFrameElement.marginWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)
    pub fn margin_width(&self) -> JsString {
        self.inner.get("marginWidth").as_::<JsString>()
    }

    /// Setter of the `marginWidth` attribute.
    /// [`HTMLFrameElement.marginWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)
    pub fn set_margin_width(&mut self, value: &JsString) {
        self.inner.set("marginWidth", value);
    }
}
