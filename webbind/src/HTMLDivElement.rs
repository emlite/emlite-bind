use super::*;

/// The HTMLDivElement class.
/// [`HTMLDivElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDivElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDivElement {
    fn from_val(v: &Any) -> Self {
        HTMLDivElement {
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
impl core::ops::Deref for HTMLDivElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDivElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLDivElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLDivElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLDivElement> for Any {
    fn from(s: HTMLDivElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLDivElement> for Any {
    fn from(s: &HTMLDivElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDivElement);

impl HTMLDivElement {
    /// The `new HTMLDivElement(..)` constructor, creating a new HTMLDivElement instance
    pub fn new() -> HTMLDivElement {
        Self {
            inner: Any::global("HTMLDivElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLDivElement {
    /// Getter of the `align` attribute.
    /// [`HTMLDivElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLDivElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)
    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
