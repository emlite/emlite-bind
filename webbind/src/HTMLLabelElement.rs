use super::*;

/// The HTMLLabelElement class.
/// [`HTMLLabelElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLabelElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLabelElement {
    fn from_val(v: &Any) -> Self {
        HTMLLabelElement {
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
impl core::ops::Deref for HTMLLabelElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLabelElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLLabelElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLLabelElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLLabelElement> for Any {
    fn from(s: HTMLLabelElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLLabelElement> for Any {
    fn from(s: &HTMLLabelElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLabelElement);

impl HTMLLabelElement {
    /// The `new HTMLLabelElement(..)` constructor, creating a new HTMLLabelElement instance
    pub fn new() -> HTMLLabelElement {
        Self {
            inner: Any::global("HTMLLabelElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLLabelElement {
    /// Getter of the `form` attribute.
    /// [`HTMLLabelElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLLabelElement {
    /// Getter of the `htmlFor` attribute.
    /// [`HTMLLabelElement.htmlFor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)
    pub fn html_for(&self) -> JsString {
        self.inner.get("htmlFor").as_::<JsString>()
    }

    /// Setter of the `htmlFor` attribute.
    /// [`HTMLLabelElement.htmlFor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)
    pub fn set_html_for(&mut self, value: &JsString) {
        self.inner.set("htmlFor", value);
    }
}
impl HTMLLabelElement {
    /// Getter of the `control` attribute.
    /// [`HTMLLabelElement.control`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control)
    pub fn control(&self) -> HTMLElement {
        self.inner.get("control").as_::<HTMLElement>()
    }
}
