use super::*;

/// The HTMLHtmlElement class.
/// [`HTMLHtmlElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLHtmlElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHtmlElement {
    fn from_val(v: &Any) -> Self {
        HTMLHtmlElement {
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
impl core::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLHtmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLHtmlElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLHtmlElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLHtmlElement> for Any {
    fn from(s: HTMLHtmlElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLHtmlElement> for Any {
    fn from(s: &HTMLHtmlElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLHtmlElement);

impl HTMLHtmlElement {
    /// The `new HTMLHtmlElement(..)` constructor, creating a new HTMLHtmlElement instance
    pub fn new() -> HTMLHtmlElement {
        Self {
            inner: Any::global("HTMLHtmlElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLHtmlElement {
    /// Getter of the `version` attribute.
    /// [`HTMLHtmlElement.version`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)
    pub fn version(&self) -> JsString {
        self.inner.get("version").as_::<JsString>()
    }

    /// Setter of the `version` attribute.
    /// [`HTMLHtmlElement.version`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)
    pub fn set_version(&mut self, value: &JsString) {
        self.inner.set("version", value);
    }
}
