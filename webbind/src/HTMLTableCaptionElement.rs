use super::*;

/// The HTMLTableCaptionElement class.
/// [`HTMLTableCaptionElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableCaptionElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableCaptionElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableCaptionElement {
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
impl core::ops::Deref for HTMLTableCaptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableCaptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTableCaptionElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableCaptionElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableCaptionElement> for Any {
    fn from(s: HTMLTableCaptionElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableCaptionElement> for Any {
    fn from(s: &HTMLTableCaptionElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableCaptionElement);

impl HTMLTableCaptionElement {
    /// The `new HTMLTableCaptionElement(..)` constructor, creating a new HTMLTableCaptionElement instance
    pub fn new() -> HTMLTableCaptionElement {
        Self {
            inner: Any::global("HTMLTableCaptionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableCaptionElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableCaptionElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableCaptionElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)
    pub fn set_align(&mut self, value: &DOMString) {
        self.inner.set("align", value);
    }
}
