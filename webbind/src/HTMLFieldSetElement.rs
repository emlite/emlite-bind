use super::*;




/// The HTMLFieldSetElement class.
/// [`HTMLFieldSetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFieldSetElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFieldSetElement {
    fn from_val(v: &Any) -> Self {
        HTMLFieldSetElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLFieldSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFieldSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFieldSetElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFieldSetElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLFieldSetElement> for Any {
    fn from(s: HTMLFieldSetElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFieldSetElement> for Any {
    fn from(s: &HTMLFieldSetElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFieldSetElement);



impl HTMLFieldSetElement {
    /// The `new HTMLFieldSetElement(..)` constructor, creating a new HTMLFieldSetElement instance
    pub fn new() -> HTMLFieldSetElement {
        Self {
            inner: Any::global("HTMLFieldSetElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLFieldSetElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLFieldSetElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLFieldSetElement {
    /// Getter of the `form` attribute.
    /// [`HTMLFieldSetElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `name` attribute.
    /// [`HTMLFieldSetElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLFieldSetElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLFieldSetElement {
    /// Getter of the `type` attribute.
    /// [`HTMLFieldSetElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `elements` attribute.
    /// [`HTMLFieldSetElement.elements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)
    pub fn elements(&self) -> HTMLCollection {
        self.inner.get("elements").as_::<HTMLCollection>()
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLFieldSetElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLFieldSetElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }

}
impl HTMLFieldSetElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLFieldSetElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }

}
impl HTMLFieldSetElement {
    /// The checkValidity method.
    /// [`HTMLFieldSetElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)
    pub fn check_validity(&self, ) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLFieldSetElement {
    /// The reportValidity method.
    /// [`HTMLFieldSetElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)
    pub fn report_validity(&self, ) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLFieldSetElement {
    /// The setCustomValidity method.
    /// [`HTMLFieldSetElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &JsString) -> Undefined {
        self.inner.call("setCustomValidity", &[error.into(), ]).as_::<Undefined>()
    }
}
