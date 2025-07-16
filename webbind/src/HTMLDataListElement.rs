use super::*;

/// The HTMLDataListElement class.
/// [`HTMLDataListElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDataListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDataListElement {
    fn from_val(v: &Any) -> Self {
        HTMLDataListElement {
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
impl core::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDataListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLDataListElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLDataListElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLDataListElement> for Any {
    fn from(s: HTMLDataListElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLDataListElement> for Any {
    fn from(s: &HTMLDataListElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDataListElement);

impl HTMLDataListElement {
    /// The `new HTMLDataListElement(..)` constructor, creating a new HTMLDataListElement instance
    pub fn new() -> HTMLDataListElement {
        Self {
            inner: Any::global("HTMLDataListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDataListElement {
    /// Getter of the `options` attribute.
    /// [`HTMLDataListElement.options`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement/options)
    pub fn options(&self) -> HTMLCollection {
        self.inner.get("options").as_::<HTMLCollection>()
    }
}
