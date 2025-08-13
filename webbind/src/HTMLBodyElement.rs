use super::*;




/// The HTMLBodyElement class.
/// [`HTMLBodyElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLBodyElement {
    inner: HTMLElement,
}

impl FromVal for HTMLBodyElement {
    fn from_val(v: &Any) -> Self {
        HTMLBodyElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLBodyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLBodyElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLBodyElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLBodyElement> for Any {
    fn from(s: HTMLBodyElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLBodyElement> for Any {
    fn from(s: &HTMLBodyElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLBodyElement);



impl HTMLBodyElement {
    /// The `new HTMLBodyElement(..)` constructor, creating a new HTMLBodyElement instance
    pub fn new() -> HTMLBodyElement {
        Self {
            inner: Any::global("HTMLBodyElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLBodyElement {
    /// Getter of the `onorientationchange` attribute.
    /// [`HTMLBodyElement.onorientationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onorientationchange)
    pub fn onorientationchange(&self) -> Any {
        self.inner.get("onorientationchange").as_::<Any>()
    }

    /// Setter of the `onorientationchange` attribute.
    /// [`HTMLBodyElement.onorientationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onorientationchange)
    pub fn set_onorientationchange(&mut self, value: &Any) {
        self.inner.set("onorientationchange", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `text` attribute.
    /// [`HTMLBodyElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    /// [`HTMLBodyElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `link` attribute.
    /// [`HTMLBodyElement.link`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)
    pub fn link(&self) -> JsString {
        self.inner.get("link").as_::<JsString>()
    }

    /// Setter of the `link` attribute.
    /// [`HTMLBodyElement.link`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)
    pub fn set_link(&mut self, value: &JsString) {
        self.inner.set("link", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `vLink` attribute.
    /// [`HTMLBodyElement.vLink`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)
    pub fn v_link(&self) -> JsString {
        self.inner.get("vLink").as_::<JsString>()
    }

    /// Setter of the `vLink` attribute.
    /// [`HTMLBodyElement.vLink`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)
    pub fn set_v_link(&mut self, value: &JsString) {
        self.inner.set("vLink", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `aLink` attribute.
    /// [`HTMLBodyElement.aLink`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)
    pub fn a_link(&self) -> JsString {
        self.inner.get("aLink").as_::<JsString>()
    }

    /// Setter of the `aLink` attribute.
    /// [`HTMLBodyElement.aLink`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)
    pub fn set_a_link(&mut self, value: &JsString) {
        self.inner.set("aLink", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `bgColor` attribute.
    /// [`HTMLBodyElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)
    pub fn bg_color(&self) -> JsString {
        self.inner.get("bgColor").as_::<JsString>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`HTMLBodyElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)
    pub fn set_bg_color(&mut self, value: &JsString) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `background` attribute.
    /// [`HTMLBodyElement.background`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)
    pub fn background(&self) -> JsString {
        self.inner.get("background").as_::<JsString>()
    }

    /// Setter of the `background` attribute.
    /// [`HTMLBodyElement.background`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)
    pub fn set_background(&mut self, value: &JsString) {
        self.inner.set("background", value);
    }
}
impl HTMLBodyElement {
    /// Getter of the `onportalactivate` attribute.
    /// [`HTMLBodyElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onportalactivate)
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    /// Setter of the `onportalactivate` attribute.
    /// [`HTMLBodyElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onportalactivate)
    pub fn set_onportalactivate(&mut self, value: &Any) {
        self.inner.set("onportalactivate", value);
    }
}
