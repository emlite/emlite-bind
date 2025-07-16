use super::*;

/// The HTMLParamElement class.
/// [`HTMLParamElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLParamElement {
    inner: HTMLElement,
}
impl FromVal for HTMLParamElement {
    fn from_val(v: &Any) -> Self {
        HTMLParamElement {
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
impl core::ops::Deref for HTMLParamElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLParamElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLParamElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLParamElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLParamElement> for Any {
    fn from(s: HTMLParamElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLParamElement> for Any {
    fn from(s: &HTMLParamElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLParamElement);

impl HTMLParamElement {
    /// The `new HTMLParamElement(..)` constructor, creating a new HTMLParamElement instance
    pub fn new() -> HTMLParamElement {
        Self {
            inner: Any::global("HTMLParamElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLParamElement {
    /// Getter of the `name` attribute.
    /// [`HTMLParamElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLParamElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)
    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLParamElement {
    /// Getter of the `value` attribute.
    /// [`HTMLParamElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)
    pub fn value(&self) -> String {
        self.inner.get("value").as_::<String>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLParamElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)
    pub fn set_value(&mut self, value: &str) {
        self.inner.set("value", value);
    }
}
impl HTMLParamElement {
    /// Getter of the `type` attribute.
    /// [`HTMLParamElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLParamElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)
    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl HTMLParamElement {
    /// Getter of the `valueType` attribute.
    /// [`HTMLParamElement.valueType`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)
    pub fn value_type(&self) -> String {
        self.inner.get("valueType").as_::<String>()
    }

    /// Setter of the `valueType` attribute.
    /// [`HTMLParamElement.valueType`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)
    pub fn set_value_type(&mut self, value: &str) {
        self.inner.set("valueType", value);
    }
}
