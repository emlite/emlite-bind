use super::*;

/// The HTMLBRElement class.
/// [`HTMLBRElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLBRElement {
    inner: HTMLElement,
}
impl FromVal for HTMLBRElement {
    fn from_val(v: &Any) -> Self {
        HTMLBRElement {
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
impl core::ops::Deref for HTMLBRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLBRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLBRElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLBRElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLBRElement> for Any {
    fn from(s: HTMLBRElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLBRElement> for Any {
    fn from(s: &HTMLBRElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLBRElement);

impl HTMLBRElement {
    /// The `new HTMLBRElement(..)` constructor, creating a new HTMLBRElement instance
    pub fn new() -> HTMLBRElement {
        Self {
            inner: Any::global("HTMLBRElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLBRElement {
    /// Getter of the `clear` attribute.
    /// [`HTMLBRElement.clear`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)
    pub fn clear(&self) -> String {
        self.inner.get("clear").as_::<String>()
    }

    /// Setter of the `clear` attribute.
    /// [`HTMLBRElement.clear`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)
    pub fn set_clear(&mut self, value: &str) {
        self.inner.set("clear", value);
    }
}
