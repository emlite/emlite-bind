use super::*;

/// The HTMLUListElement class.
/// [`HTMLUListElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLUListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLUListElement {
    fn from_val(v: &Any) -> Self {
        HTMLUListElement {
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
impl core::ops::Deref for HTMLUListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLUListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLUListElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLUListElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLUListElement> for Any {
    fn from(s: HTMLUListElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLUListElement> for Any {
    fn from(s: &HTMLUListElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLUListElement);

impl HTMLUListElement {
    /// The `new HTMLUListElement(..)` constructor, creating a new HTMLUListElement instance
    pub fn new() -> HTMLUListElement {
        Self {
            inner: Any::global("HTMLUListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLUListElement {
    /// Getter of the `compact` attribute.
    /// [`HTMLUListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    /// Setter of the `compact` attribute.
    /// [`HTMLUListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)
    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
impl HTMLUListElement {
    /// Getter of the `type` attribute.
    /// [`HTMLUListElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLUListElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)
    pub fn set_type_(&mut self, value: &DOMString) {
        self.inner.set("type", value);
    }
}
