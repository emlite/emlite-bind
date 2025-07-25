use super::*;

/// The HTMLOutputElement class.
/// [`HTMLOutputElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOutputElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOutputElement {
    fn from_val(v: &Any) -> Self {
        HTMLOutputElement {
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
impl core::ops::Deref for HTMLOutputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOutputElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLOutputElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLOutputElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLOutputElement> for Any {
    fn from(s: HTMLOutputElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLOutputElement> for Any {
    fn from(s: &HTMLOutputElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOutputElement);

impl HTMLOutputElement {
    /// The `new HTMLOutputElement(..)` constructor, creating a new HTMLOutputElement instance
    pub fn new() -> HTMLOutputElement {
        Self {
            inner: Any::global("HTMLOutputElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOutputElement {
    /// Getter of the `htmlFor` attribute.
    /// [`HTMLOutputElement.htmlFor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)
    pub fn html_for(&self) -> DOMTokenList {
        self.inner.get("htmlFor").as_::<DOMTokenList>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `form` attribute.
    /// [`HTMLOutputElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `name` attribute.
    /// [`HTMLOutputElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLOutputElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)
    pub fn set_name(&mut self, value: &DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLOutputElement {
    /// Getter of the `type` attribute.
    /// [`HTMLOutputElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `defaultValue` attribute.
    /// [`HTMLOutputElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)
    pub fn default_value(&self) -> DOMString {
        self.inner.get("defaultValue").as_::<DOMString>()
    }

    /// Setter of the `defaultValue` attribute.
    /// [`HTMLOutputElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)
    pub fn set_default_value(&mut self, value: &DOMString) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLOutputElement {
    /// Getter of the `value` attribute.
    /// [`HTMLOutputElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLOutputElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)
    pub fn set_value(&mut self, value: &DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLOutputElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLOutputElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLOutputElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLOutputElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)
    pub fn validation_message(&self) -> DOMString {
        self.inner.get("validationMessage").as_::<DOMString>()
    }
}
impl HTMLOutputElement {
    /// The checkValidity method.
    /// [`HTMLOutputElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLOutputElement {
    /// The reportValidity method.
    /// [`HTMLOutputElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLOutputElement {
    /// The setCustomValidity method.
    /// [`HTMLOutputElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &DOMString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLOutputElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLOutputElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
