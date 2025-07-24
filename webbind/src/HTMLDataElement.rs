use super::*;

/// The HTMLDataElement class.
/// [`HTMLDataElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDataElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDataElement {
    fn from_val(v: &Any) -> Self {
        HTMLDataElement {
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
impl core::ops::Deref for HTMLDataElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDataElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLDataElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLDataElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLDataElement> for Any {
    fn from(s: HTMLDataElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLDataElement> for Any {
    fn from(s: &HTMLDataElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDataElement);

impl HTMLDataElement {
    /// The `new HTMLDataElement(..)` constructor, creating a new HTMLDataElement instance
    pub fn new() -> HTMLDataElement {
        Self {
            inner: Any::global("HTMLDataElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLDataElement {
    /// Getter of the `value` attribute.
    /// [`HTMLDataElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLDataElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)
    pub fn set_value(&mut self, value: &DOMString) {
        self.inner.set("value", value);
    }
}
