use super::*;

/// The HTMLTimeElement class.
/// [`HTMLTimeElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTimeElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTimeElement {
    fn from_val(v: &Any) -> Self {
        HTMLTimeElement {
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
impl core::ops::Deref for HTMLTimeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTimeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTimeElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTimeElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTimeElement> for Any {
    fn from(s: HTMLTimeElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTimeElement> for Any {
    fn from(s: &HTMLTimeElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTimeElement);

impl HTMLTimeElement {
    /// The `new HTMLTimeElement(..)` constructor, creating a new HTMLTimeElement instance
    pub fn new() -> HTMLTimeElement {
        Self {
            inner: Any::global("HTMLTimeElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLTimeElement {
    /// Getter of the `dateTime` attribute.
    /// [`HTMLTimeElement.dateTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)
    pub fn date_time(&self) -> DOMString {
        self.inner.get("dateTime").as_::<DOMString>()
    }

    /// Setter of the `dateTime` attribute.
    /// [`HTMLTimeElement.dateTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)
    pub fn set_date_time(&mut self, value: &DOMString) {
        self.inner.set("dateTime", value);
    }
}
