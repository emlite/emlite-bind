use super::*;

/// The HTMLLIElement class.
/// [`HTMLLIElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLIElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLIElement {
    fn from_val(v: &Any) -> Self {
        HTMLLIElement {
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
impl core::ops::Deref for HTMLLIElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLIElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLLIElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLLIElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLLIElement> for Any {
    fn from(s: HTMLLIElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLLIElement> for Any {
    fn from(s: &HTMLLIElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLIElement);

impl HTMLLIElement {
    /// The `new HTMLLIElement(..)` constructor, creating a new HTMLLIElement instance
    pub fn new() -> HTMLLIElement {
        Self {
            inner: Any::global("HTMLLIElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLLIElement {
    /// Getter of the `value` attribute.
    /// [`HTMLLIElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)
    pub fn value(&self) -> i32 {
        self.inner.get("value").as_::<i32>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLLIElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)
    pub fn set_value(&mut self, value: i32) {
        self.inner.set("value", value);
    }
}
impl HTMLLIElement {
    /// Getter of the `type` attribute.
    /// [`HTMLLIElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLLIElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
