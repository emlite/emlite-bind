use super::*;




/// The HTMLSelectedContentElement class.
/// [`HTMLSelectedContentElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectedContentElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSelectedContentElement {
    inner: HTMLElement,
}

impl FromVal for HTMLSelectedContentElement {
    fn from_val(v: &Any) -> Self {
        HTMLSelectedContentElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLSelectedContentElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLSelectedContentElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLSelectedContentElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLSelectedContentElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLSelectedContentElement> for Any {
    fn from(s: HTMLSelectedContentElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLSelectedContentElement> for Any {
    fn from(s: &HTMLSelectedContentElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLSelectedContentElement);



impl HTMLSelectedContentElement {
    /// The `new HTMLSelectedContentElement(..)` constructor, creating a new HTMLSelectedContentElement instance
    pub fn new() -> HTMLSelectedContentElement {
        Self {
            inner: Any::global("HTMLSelectedContentElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
