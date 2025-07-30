use super::*;

/// The HTMLTitleElement class.
/// [`HTMLTitleElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTitleElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTitleElement {
    fn from_val(v: &Any) -> Self {
        HTMLTitleElement {
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
impl core::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTitleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTitleElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTitleElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTitleElement> for Any {
    fn from(s: HTMLTitleElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTitleElement> for Any {
    fn from(s: &HTMLTitleElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTitleElement);

impl HTMLTitleElement {
    /// The `new HTMLTitleElement(..)` constructor, creating a new HTMLTitleElement instance
    pub fn new() -> HTMLTitleElement {
        Self {
            inner: Any::global("HTMLTitleElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTitleElement {
    /// Getter of the `text` attribute.
    /// [`HTMLTitleElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    /// [`HTMLTitleElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
