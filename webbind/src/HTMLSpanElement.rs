use super::*;




/// The HTMLSpanElement class.
/// [`HTMLSpanElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSpanElement {
    inner: HTMLElement,
}

impl FromVal for HTMLSpanElement {
    fn from_val(v: &Any) -> Self {
        HTMLSpanElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLSpanElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLSpanElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLSpanElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLSpanElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLSpanElement> for Any {
    fn from(s: HTMLSpanElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLSpanElement> for Any {
    fn from(s: &HTMLSpanElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLSpanElement);



impl HTMLSpanElement {
    /// The `new HTMLSpanElement(..)` constructor, creating a new HTMLSpanElement instance
    pub fn new() -> HTMLSpanElement {
        Self {
            inner: Any::global("HTMLSpanElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
