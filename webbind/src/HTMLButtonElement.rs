use super::*;

#[derive(Clone, Debug)]
pub struct HTMLButtonElement {
    inner: HTMLElement,
}
impl FromVal for HTMLButtonElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLButtonElement {
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
impl std::ops::Deref for HTMLButtonElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLButtonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLButtonElement> for emlite::Val {
    fn from(s: HTMLButtonElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLButtonElement {
    pub fn new() -> HTMLButtonElement {
        Self {
            inner: emlite::Val::global("HTMLButtonElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLButtonElement {
    pub fn command(&self) -> jsbind::DOMString {
        self.inner.get("command").as_::<jsbind::DOMString>()
    }

    pub fn set_command(&mut self, value: jsbind::DOMString) {
        self.inner.set("command", value);
    }
}
impl HTMLButtonElement {
    pub fn command_for_element(&self) -> Element {
        self.inner.get("commandForElement").as_::<Element>()
    }

    pub fn set_command_for_element(&mut self, value: Element) {
        self.inner.set("commandForElement", value);
    }
}
impl HTMLButtonElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLButtonElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLButtonElement {
    pub fn form_action(&self) -> jsbind::USVString {
        self.inner.get("formAction").as_::<jsbind::USVString>()
    }

    pub fn set_form_action(&mut self, value: jsbind::USVString) {
        self.inner.set("formAction", value);
    }
}
impl HTMLButtonElement {
    pub fn form_enctype(&self) -> jsbind::DOMString {
        self.inner.get("formEnctype").as_::<jsbind::DOMString>()
    }

    pub fn set_form_enctype(&mut self, value: jsbind::DOMString) {
        self.inner.set("formEnctype", value);
    }
}
impl HTMLButtonElement {
    pub fn form_method(&self) -> jsbind::DOMString {
        self.inner.get("formMethod").as_::<jsbind::DOMString>()
    }

    pub fn set_form_method(&mut self, value: jsbind::DOMString) {
        self.inner.set("formMethod", value);
    }
}
impl HTMLButtonElement {
    pub fn form_no_validate(&self) -> bool {
        self.inner.get("formNoValidate").as_::<bool>()
    }

    pub fn set_form_no_validate(&mut self, value: bool) {
        self.inner.set("formNoValidate", value);
    }
}
impl HTMLButtonElement {
    pub fn form_target(&self) -> jsbind::DOMString {
        self.inner.get("formTarget").as_::<jsbind::DOMString>()
    }

    pub fn set_form_target(&mut self, value: jsbind::DOMString) {
        self.inner.set("formTarget", value);
    }
}
impl HTMLButtonElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLButtonElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLButtonElement {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLButtonElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLButtonElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLButtonElement {
    pub fn validation_message(&self) -> jsbind::DOMString {
        self.inner
            .get("validationMessage")
            .as_::<jsbind::DOMString>()
    }
}
impl HTMLButtonElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLButtonElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLButtonElement {
    pub fn set_custom_validity(&self, error: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLButtonElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLButtonElement {
    pub fn popover_target_element(&self) -> Element {
        self.inner.get("popoverTargetElement").as_::<Element>()
    }

    pub fn set_popover_target_element(&mut self, value: Element) {
        self.inner.set("popoverTargetElement", value);
    }
}
impl HTMLButtonElement {
    pub fn popover_target_action(&self) -> jsbind::DOMString {
        self.inner
            .get("popoverTargetAction")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_popover_target_action(&mut self, value: jsbind::DOMString) {
        self.inner.set("popoverTargetAction", value);
    }
}
