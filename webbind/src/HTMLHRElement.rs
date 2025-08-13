use super::*;




/// The HTMLHRElement class.
/// [`HTMLHRElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLHRElement {
    inner: HTMLElement,
}

impl FromVal for HTMLHRElement {
    fn from_val(v: &Any) -> Self {
        HTMLHRElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLHRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLHRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLHRElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLHRElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLHRElement> for Any {
    fn from(s: HTMLHRElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLHRElement> for Any {
    fn from(s: &HTMLHRElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLHRElement);



impl HTMLHRElement {
    /// The `new HTMLHRElement(..)` constructor, creating a new HTMLHRElement instance
    pub fn new() -> HTMLHRElement {
        Self {
            inner: Any::global("HTMLHRElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLHRElement {
    /// Getter of the `align` attribute.
    /// [`HTMLHRElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLHRElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLHRElement {
    /// Getter of the `color` attribute.
    /// [`HTMLHRElement.color`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)
    pub fn color(&self) -> JsString {
        self.inner.get("color").as_::<JsString>()
    }

    /// Setter of the `color` attribute.
    /// [`HTMLHRElement.color`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)
    pub fn set_color(&mut self, value: &JsString) {
        self.inner.set("color", value);
    }
}
impl HTMLHRElement {
    /// Getter of the `noShade` attribute.
    /// [`HTMLHRElement.noShade`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)
    pub fn no_shade(&self) -> bool {
        self.inner.get("noShade").as_::<bool>()
    }

    /// Setter of the `noShade` attribute.
    /// [`HTMLHRElement.noShade`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)
    pub fn set_no_shade(&mut self, value: bool) {
        self.inner.set("noShade", value);
    }
}
impl HTMLHRElement {
    /// Getter of the `size` attribute.
    /// [`HTMLHRElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)
    pub fn size(&self) -> JsString {
        self.inner.get("size").as_::<JsString>()
    }

    /// Setter of the `size` attribute.
    /// [`HTMLHRElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)
    pub fn set_size(&mut self, value: &JsString) {
        self.inner.set("size", value);
    }
}
impl HTMLHRElement {
    /// Getter of the `width` attribute.
    /// [`HTMLHRElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLHRElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
