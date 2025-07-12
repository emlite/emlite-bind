use super::*;

#[derive(Clone, Debug)]
pub struct HTMLFieldSetElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFieldSetElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFieldSetElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HTMLFieldSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLFieldSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLFieldSetElement> for emlite::Val {
    fn from(s: HTMLFieldSetElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLFieldSetElement {
    pub fn new() -> HTMLFieldSetElement {
        Self {
            inner: emlite::Val::global("HTMLFieldSetElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFieldSetElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLFieldSetElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLFieldSetElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLFieldSetElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
impl HTMLFieldSetElement {
    pub fn elements(&self) -> HTMLCollection {
        self.inner.get("elements").as_::<HTMLCollection>()
    }
}
impl HTMLFieldSetElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLFieldSetElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLFieldSetElement {
    pub fn validation_message(&self) -> jsbind::DOMString {
        self.inner
            .get("validationMessage")
            .as_::<jsbind::DOMString>()
    }
}
impl HTMLFieldSetElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLFieldSetElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLFieldSetElement {
    pub fn set_custom_validity(&self, error: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<jsbind::Undefined>()
    }
}
