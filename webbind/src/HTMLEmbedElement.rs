use super::*;

/// The HTMLEmbedElement class.
/// [`HTMLEmbedElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLEmbedElement {
    inner: HTMLElement,
}
impl FromVal for HTMLEmbedElement {
    fn from_val(v: &Any) -> Self {
        HTMLEmbedElement {
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
impl core::ops::Deref for HTMLEmbedElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLEmbedElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLEmbedElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLEmbedElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLEmbedElement> for Any {
    fn from(s: HTMLEmbedElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLEmbedElement> for Any {
    fn from(s: &HTMLEmbedElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLEmbedElement);

impl HTMLEmbedElement {
    /// The `new HTMLEmbedElement(..)` constructor, creating a new HTMLEmbedElement instance
    pub fn new() -> HTMLEmbedElement {
        Self {
            inner: Any::global("HTMLEmbedElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLEmbedElement {
    /// Getter of the `src` attribute.
    /// [`HTMLEmbedElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLEmbedElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLEmbedElement {
    /// Getter of the `type` attribute.
    /// [`HTMLEmbedElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLEmbedElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLEmbedElement {
    /// Getter of the `width` attribute.
    /// [`HTMLEmbedElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLEmbedElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLEmbedElement {
    /// Getter of the `height` attribute.
    /// [`HTMLEmbedElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLEmbedElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl HTMLEmbedElement {
    /// The getSVGDocument method.
    /// [`HTMLEmbedElement.getSVGDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/getSVGDocument)
    pub fn get_svg_document(&self) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLEmbedElement {
    /// Getter of the `align` attribute.
    /// [`HTMLEmbedElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLEmbedElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLEmbedElement {
    /// Getter of the `name` attribute.
    /// [`HTMLEmbedElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLEmbedElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
