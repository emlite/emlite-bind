use super::*;

/// The HTMLFontElement class.
/// [`HTMLFontElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFontElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFontElement {
    fn from_val(v: &Any) -> Self {
        HTMLFontElement {
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

impl core::ops::Deref for HTMLFontElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFontElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFontElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFontElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLFontElement> for Any {
    fn from(s: HTMLFontElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFontElement> for Any {
    fn from(s: &HTMLFontElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFontElement);

impl HTMLFontElement {
    /// Getter of the `color` attribute.
    /// [`HTMLFontElement.color`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)
    pub fn color(&self) -> JsString {
        self.inner.get("color").as_::<JsString>()
    }

    /// Setter of the `color` attribute.
    /// [`HTMLFontElement.color`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)
    pub fn set_color(&mut self, value: &JsString) {
        self.inner.set("color", value);
    }
}
impl HTMLFontElement {
    /// Getter of the `face` attribute.
    /// [`HTMLFontElement.face`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)
    pub fn face(&self) -> JsString {
        self.inner.get("face").as_::<JsString>()
    }

    /// Setter of the `face` attribute.
    /// [`HTMLFontElement.face`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)
    pub fn set_face(&mut self, value: &JsString) {
        self.inner.set("face", value);
    }
}
impl HTMLFontElement {
    /// Getter of the `size` attribute.
    /// [`HTMLFontElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)
    pub fn size(&self) -> JsString {
        self.inner.get("size").as_::<JsString>()
    }

    /// Setter of the `size` attribute.
    /// [`HTMLFontElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)
    pub fn set_size(&mut self, value: &JsString) {
        self.inner.set("size", value);
    }
}

impl HTMLFontElement {
    /// The `new HTMLFontElement(..)` constructor, creating a new HTMLFontElement instance
    pub fn new() -> HTMLFontElement {
        Self {
            inner: Any::global("HTMLFontElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
