use super::*;

/// The HTMLLegendElement class.
/// [`HTMLLegendElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLegendElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLegendElement {
    fn from_val(v: &Any) -> Self {
        HTMLLegendElement {
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
impl core::ops::Deref for HTMLLegendElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLegendElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLLegendElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLLegendElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLLegendElement> for Any {
    fn from(s: HTMLLegendElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLLegendElement> for Any {
    fn from(s: &HTMLLegendElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLegendElement);

impl HTMLLegendElement {
    /// The `new HTMLLegendElement(..)` constructor, creating a new HTMLLegendElement instance
    pub fn new() -> HTMLLegendElement {
        Self {
            inner: Any::global("HTMLLegendElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLLegendElement {
    /// Getter of the `form` attribute.
    /// [`HTMLLegendElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLLegendElement {
    /// Getter of the `align` attribute.
    /// [`HTMLLegendElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLLegendElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
