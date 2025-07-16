use super::*;

/// The HTMLParagraphElement class.
/// [`HTMLParagraphElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLParagraphElement {
    inner: HTMLElement,
}
impl FromVal for HTMLParagraphElement {
    fn from_val(v: &Any) -> Self {
        HTMLParagraphElement {
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
impl core::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLParagraphElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLParagraphElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLParagraphElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLParagraphElement> for Any {
    fn from(s: HTMLParagraphElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLParagraphElement> for Any {
    fn from(s: &HTMLParagraphElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLParagraphElement);

impl HTMLParagraphElement {
    /// The `new HTMLParagraphElement(..)` constructor, creating a new HTMLParagraphElement instance
    pub fn new() -> HTMLParagraphElement {
        Self {
            inner: Any::global("HTMLParagraphElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLParagraphElement {
    /// Getter of the `align` attribute.
    /// [`HTMLParagraphElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLParagraphElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)
    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
