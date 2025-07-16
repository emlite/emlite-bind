use super::*;

/// The HTMLDirectoryElement class.
/// [`HTMLDirectoryElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDirectoryElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDirectoryElement {
    fn from_val(v: &Any) -> Self {
        HTMLDirectoryElement {
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
impl core::ops::Deref for HTMLDirectoryElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDirectoryElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLDirectoryElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLDirectoryElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLDirectoryElement> for Any {
    fn from(s: HTMLDirectoryElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLDirectoryElement> for Any {
    fn from(s: &HTMLDirectoryElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDirectoryElement);

impl HTMLDirectoryElement {
    /// The `new HTMLDirectoryElement(..)` constructor, creating a new HTMLDirectoryElement instance
    pub fn new() -> HTMLDirectoryElement {
        Self {
            inner: Any::global("HTMLDirectoryElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDirectoryElement {
    /// Getter of the `compact` attribute.
    /// [`HTMLDirectoryElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    /// Setter of the `compact` attribute.
    /// [`HTMLDirectoryElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)
    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
