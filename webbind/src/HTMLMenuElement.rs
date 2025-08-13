use super::*;




/// The HTMLMenuElement class.
/// [`HTMLMenuElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMenuElement {
    inner: HTMLElement,
}

impl FromVal for HTMLMenuElement {
    fn from_val(v: &Any) -> Self {
        HTMLMenuElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLMenuElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLMenuElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLMenuElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLMenuElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLMenuElement> for Any {
    fn from(s: HTMLMenuElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLMenuElement> for Any {
    fn from(s: &HTMLMenuElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLMenuElement);



impl HTMLMenuElement {
    /// The `new HTMLMenuElement(..)` constructor, creating a new HTMLMenuElement instance
    pub fn new() -> HTMLMenuElement {
        Self {
            inner: Any::global("HTMLMenuElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLMenuElement {
    /// Getter of the `compact` attribute.
    /// [`HTMLMenuElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    /// Setter of the `compact` attribute.
    /// [`HTMLMenuElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)
    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
