use super::*;




/// The HTMLFencedFrameElement class.
/// [`HTMLFencedFrameElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFencedFrameElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFencedFrameElement {
    fn from_val(v: &Any) -> Self {
        HTMLFencedFrameElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLFencedFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFencedFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFencedFrameElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFencedFrameElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLFencedFrameElement> for Any {
    fn from(s: HTMLFencedFrameElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFencedFrameElement> for Any {
    fn from(s: &HTMLFencedFrameElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFencedFrameElement);



impl HTMLFencedFrameElement {
    /// The `new HTMLFencedFrameElement(..)` constructor, creating a new HTMLFencedFrameElement instance
    pub fn new() -> HTMLFencedFrameElement {
        Self {
            inner: Any::global("HTMLFencedFrameElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLFencedFrameElement {
    /// Getter of the `config` attribute.
    /// [`HTMLFencedFrameElement.config`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/config)
    pub fn config(&self) -> FencedFrameConfig {
        self.inner.get("config").as_::<FencedFrameConfig>()
    }

    /// Setter of the `config` attribute.
    /// [`HTMLFencedFrameElement.config`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/config)
    pub fn set_config(&mut self, value: &FencedFrameConfig) {
        self.inner.set("config", value);
    }
}
impl HTMLFencedFrameElement {
    /// Getter of the `width` attribute.
    /// [`HTMLFencedFrameElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLFencedFrameElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLFencedFrameElement {
    /// Getter of the `height` attribute.
    /// [`HTMLFencedFrameElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLFencedFrameElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl HTMLFencedFrameElement {
    /// Getter of the `sandbox` attribute.
    /// [`HTMLFencedFrameElement.sandbox`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/sandbox)
    pub fn sandbox(&self) -> DOMTokenList {
        self.inner.get("sandbox").as_::<DOMTokenList>()
    }

}
impl HTMLFencedFrameElement {
    /// Getter of the `allow` attribute.
    /// [`HTMLFencedFrameElement.allow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/allow)
    pub fn allow(&self) -> JsString {
        self.inner.get("allow").as_::<JsString>()
    }

    /// Setter of the `allow` attribute.
    /// [`HTMLFencedFrameElement.allow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFencedFrameElement/allow)
    pub fn set_allow(&mut self, value: &JsString) {
        self.inner.set("allow", value);
    }
}
