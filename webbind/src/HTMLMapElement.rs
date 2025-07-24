use super::*;

/// The HTMLMapElement class.
/// [`HTMLMapElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMapElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMapElement {
    fn from_val(v: &Any) -> Self {
        HTMLMapElement {
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
impl core::ops::Deref for HTMLMapElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMapElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLMapElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLMapElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLMapElement> for Any {
    fn from(s: HTMLMapElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLMapElement> for Any {
    fn from(s: &HTMLMapElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMapElement);

impl HTMLMapElement {
    /// The `new HTMLMapElement(..)` constructor, creating a new HTMLMapElement instance
    pub fn new() -> HTMLMapElement {
        Self {
            inner: Any::global("HTMLMapElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLMapElement {
    /// Getter of the `name` attribute.
    /// [`HTMLMapElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLMapElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)
    pub fn set_name(&mut self, value: &DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLMapElement {
    /// Getter of the `areas` attribute.
    /// [`HTMLMapElement.areas`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/areas)
    pub fn areas(&self) -> HTMLCollection {
        self.inner.get("areas").as_::<HTMLCollection>()
    }
}
