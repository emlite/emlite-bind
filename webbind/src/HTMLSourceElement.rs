use super::*;




/// The HTMLSourceElement class.
/// [`HTMLSourceElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSourceElement {
    inner: HTMLElement,
}

impl FromVal for HTMLSourceElement {
    fn from_val(v: &Any) -> Self {
        HTMLSourceElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLSourceElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLSourceElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLSourceElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLSourceElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLSourceElement> for Any {
    fn from(s: HTMLSourceElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLSourceElement> for Any {
    fn from(s: &HTMLSourceElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLSourceElement);



impl HTMLSourceElement {
    /// The `new HTMLSourceElement(..)` constructor, creating a new HTMLSourceElement instance
    pub fn new() -> HTMLSourceElement {
        Self {
            inner: Any::global("HTMLSourceElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLSourceElement {
    /// Getter of the `src` attribute.
    /// [`HTMLSourceElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLSourceElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `type` attribute.
    /// [`HTMLSourceElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLSourceElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `srcset` attribute.
    /// [`HTMLSourceElement.srcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)
    pub fn srcset(&self) -> JsString {
        self.inner.get("srcset").as_::<JsString>()
    }

    /// Setter of the `srcset` attribute.
    /// [`HTMLSourceElement.srcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)
    pub fn set_srcset(&mut self, value: &JsString) {
        self.inner.set("srcset", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `sizes` attribute.
    /// [`HTMLSourceElement.sizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)
    pub fn sizes(&self) -> JsString {
        self.inner.get("sizes").as_::<JsString>()
    }

    /// Setter of the `sizes` attribute.
    /// [`HTMLSourceElement.sizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)
    pub fn set_sizes(&mut self, value: &JsString) {
        self.inner.set("sizes", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `media` attribute.
    /// [`HTMLSourceElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    /// [`HTMLSourceElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `width` attribute.
    /// [`HTMLSourceElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLSourceElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/width)
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLSourceElement {
    /// Getter of the `height` attribute.
    /// [`HTMLSourceElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLSourceElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/height)
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
