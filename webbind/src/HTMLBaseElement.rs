use super::*;

/// The HTMLBaseElement class.
/// [`HTMLBaseElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLBaseElement {
    inner: HTMLElement,
}
impl FromVal for HTMLBaseElement {
    fn from_val(v: &Any) -> Self {
        HTMLBaseElement {
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
impl core::ops::Deref for HTMLBaseElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLBaseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLBaseElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLBaseElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLBaseElement> for Any {
    fn from(s: HTMLBaseElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLBaseElement> for Any {
    fn from(s: &HTMLBaseElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLBaseElement);

impl HTMLBaseElement {
    /// The `new HTMLBaseElement(..)` constructor, creating a new HTMLBaseElement instance
    pub fn new() -> HTMLBaseElement {
        Self {
            inner: Any::global("HTMLBaseElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLBaseElement {
    /// Getter of the `href` attribute.
    /// [`HTMLBaseElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)
    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }

    /// Setter of the `href` attribute.
    /// [`HTMLBaseElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)
    pub fn set_href(&mut self, value: &str) {
        self.inner.set("href", value);
    }
}
impl HTMLBaseElement {
    /// Getter of the `target` attribute.
    /// [`HTMLBaseElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)
    pub fn target(&self) -> String {
        self.inner.get("target").as_::<String>()
    }

    /// Setter of the `target` attribute.
    /// [`HTMLBaseElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)
    pub fn set_target(&mut self, value: &str) {
        self.inner.set("target", value);
    }
}
