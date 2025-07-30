use super::*;

/// The HTMLOptGroupElement class.
/// [`HTMLOptGroupElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptGroupElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOptGroupElement {
    fn from_val(v: &Any) -> Self {
        HTMLOptGroupElement {
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
impl core::ops::Deref for HTMLOptGroupElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOptGroupElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLOptGroupElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLOptGroupElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLOptGroupElement> for Any {
    fn from(s: HTMLOptGroupElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLOptGroupElement> for Any {
    fn from(s: &HTMLOptGroupElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOptGroupElement);

impl HTMLOptGroupElement {
    /// The `new HTMLOptGroupElement(..)` constructor, creating a new HTMLOptGroupElement instance
    pub fn new() -> HTMLOptGroupElement {
        Self {
            inner: Any::global("HTMLOptGroupElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOptGroupElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLOptGroupElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLOptGroupElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLOptGroupElement {
    /// Getter of the `label` attribute.
    /// [`HTMLOptGroupElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`HTMLOptGroupElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
