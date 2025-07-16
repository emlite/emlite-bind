use super::*;

/// The HTMLPictureElement class.
/// [`HTMLPictureElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPictureElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLPictureElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPictureElement {
    fn from_val(v: &Any) -> Self {
        HTMLPictureElement {
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
impl core::ops::Deref for HTMLPictureElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLPictureElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLPictureElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLPictureElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLPictureElement> for Any {
    fn from(s: HTMLPictureElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLPictureElement> for Any {
    fn from(s: &HTMLPictureElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLPictureElement);

impl HTMLPictureElement {
    /// The `new HTMLPictureElement(..)` constructor, creating a new HTMLPictureElement instance
    pub fn new() -> HTMLPictureElement {
        Self {
            inner: Any::global("HTMLPictureElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
