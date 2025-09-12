use super::*;

/// The HTMLDListElement class.
/// [`HTMLDListElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDListElement {
    inner: HTMLElement,
}

impl FromVal for HTMLDListElement {
    fn from_val(v: &Any) -> Self {
        HTMLDListElement {
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

impl core::ops::Deref for HTMLDListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLDListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLDListElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLDListElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLDListElement> for Any {
    fn from(s: HTMLDListElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLDListElement> for Any {
    fn from(s: &HTMLDListElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLDListElement);

impl HTMLDListElement {
    /// Getter of the `compact` attribute.
    /// [`HTMLDListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    /// Setter of the `compact` attribute.
    /// [`HTMLDListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)
    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}

impl HTMLDListElement {
    /// The `new HTMLDListElement(..)` constructor, creating a new HTMLDListElement instance
    pub fn new() -> HTMLDListElement {
        Self {
            inner: Any::global("HTMLDListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
