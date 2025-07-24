use super::*;

/// The HTMLButtonElement class.
/// [`HTMLButtonElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLButtonElement {
    inner: HTMLElement,
}
impl FromVal for HTMLButtonElement {
    fn from_val(v: &Any) -> Self {
        HTMLButtonElement {
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
impl core::ops::Deref for HTMLButtonElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLButtonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLButtonElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLButtonElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLButtonElement> for Any {
    fn from(s: HTMLButtonElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLButtonElement> for Any {
    fn from(s: &HTMLButtonElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLButtonElement);

impl HTMLButtonElement {
    /// The `new HTMLButtonElement(..)` constructor, creating a new HTMLButtonElement instance
    pub fn new() -> HTMLButtonElement {
        Self {
            inner: Any::global("HTMLButtonElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLButtonElement {
    /// Getter of the `command` attribute.
    /// [`HTMLButtonElement.command`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/command)
    pub fn command(&self) -> DOMString {
        self.inner.get("command").as_::<DOMString>()
    }

    /// Setter of the `command` attribute.
    /// [`HTMLButtonElement.command`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/command)
    pub fn set_command(&mut self, value: &DOMString) {
        self.inner.set("command", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `commandForElement` attribute.
    /// [`HTMLButtonElement.commandForElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/commandForElement)
    pub fn command_for_element(&self) -> Element {
        self.inner.get("commandForElement").as_::<Element>()
    }

    /// Setter of the `commandForElement` attribute.
    /// [`HTMLButtonElement.commandForElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/commandForElement)
    pub fn set_command_for_element(&mut self, value: &Element) {
        self.inner.set("commandForElement", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLButtonElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLButtonElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `form` attribute.
    /// [`HTMLButtonElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLButtonElement {
    /// Getter of the `formAction` attribute.
    /// [`HTMLButtonElement.formAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)
    pub fn form_action(&self) -> USVString {
        self.inner.get("formAction").as_::<USVString>()
    }

    /// Setter of the `formAction` attribute.
    /// [`HTMLButtonElement.formAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)
    pub fn set_form_action(&mut self, value: &USVString) {
        self.inner.set("formAction", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `formEnctype` attribute.
    /// [`HTMLButtonElement.formEnctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)
    pub fn form_enctype(&self) -> DOMString {
        self.inner.get("formEnctype").as_::<DOMString>()
    }

    /// Setter of the `formEnctype` attribute.
    /// [`HTMLButtonElement.formEnctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)
    pub fn set_form_enctype(&mut self, value: &DOMString) {
        self.inner.set("formEnctype", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `formMethod` attribute.
    /// [`HTMLButtonElement.formMethod`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)
    pub fn form_method(&self) -> DOMString {
        self.inner.get("formMethod").as_::<DOMString>()
    }

    /// Setter of the `formMethod` attribute.
    /// [`HTMLButtonElement.formMethod`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)
    pub fn set_form_method(&mut self, value: &DOMString) {
        self.inner.set("formMethod", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `formNoValidate` attribute.
    /// [`HTMLButtonElement.formNoValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)
    pub fn form_no_validate(&self) -> bool {
        self.inner.get("formNoValidate").as_::<bool>()
    }

    /// Setter of the `formNoValidate` attribute.
    /// [`HTMLButtonElement.formNoValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)
    pub fn set_form_no_validate(&mut self, value: bool) {
        self.inner.set("formNoValidate", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `formTarget` attribute.
    /// [`HTMLButtonElement.formTarget`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)
    pub fn form_target(&self) -> DOMString {
        self.inner.get("formTarget").as_::<DOMString>()
    }

    /// Setter of the `formTarget` attribute.
    /// [`HTMLButtonElement.formTarget`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)
    pub fn set_form_target(&mut self, value: &DOMString) {
        self.inner.set("formTarget", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `name` attribute.
    /// [`HTMLButtonElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLButtonElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)
    pub fn set_name(&mut self, value: &DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `type` attribute.
    /// [`HTMLButtonElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLButtonElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)
    pub fn set_type_(&mut self, value: &DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `value` attribute.
    /// [`HTMLButtonElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLButtonElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)
    pub fn set_value(&mut self, value: &DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLButtonElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLButtonElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLButtonElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLButtonElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLButtonElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)
    pub fn validation_message(&self) -> DOMString {
        self.inner.get("validationMessage").as_::<DOMString>()
    }
}
impl HTMLButtonElement {
    /// The checkValidity method.
    /// [`HTMLButtonElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLButtonElement {
    /// The reportValidity method.
    /// [`HTMLButtonElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLButtonElement {
    /// The setCustomValidity method.
    /// [`HTMLButtonElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &DOMString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLButtonElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLButtonElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLButtonElement {
    /// Getter of the `popoverTargetElement` attribute.
    /// [`HTMLButtonElement.popoverTargetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)
    pub fn popover_target_element(&self) -> Element {
        self.inner.get("popoverTargetElement").as_::<Element>()
    }

    /// Setter of the `popoverTargetElement` attribute.
    /// [`HTMLButtonElement.popoverTargetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)
    pub fn set_popover_target_element(&mut self, value: &Element) {
        self.inner.set("popoverTargetElement", value);
    }
}
impl HTMLButtonElement {
    /// Getter of the `popoverTargetAction` attribute.
    /// [`HTMLButtonElement.popoverTargetAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)
    pub fn popover_target_action(&self) -> DOMString {
        self.inner.get("popoverTargetAction").as_::<DOMString>()
    }

    /// Setter of the `popoverTargetAction` attribute.
    /// [`HTMLButtonElement.popoverTargetAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)
    pub fn set_popover_target_action(&mut self, value: &DOMString) {
        self.inner.set("popoverTargetAction", value);
    }
}
