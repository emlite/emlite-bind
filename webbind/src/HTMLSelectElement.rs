use super::*;

/// The HTMLSelectElement class.
/// [`HTMLSelectElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSelectElement {
    inner: HTMLElement,
}

impl FromVal for HTMLSelectElement {
    fn from_val(v: &Any) -> Self {
        HTMLSelectElement {
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

impl core::ops::Deref for HTMLSelectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLSelectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLSelectElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLSelectElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLSelectElement> for Any {
    fn from(s: HTMLSelectElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLSelectElement> for Any {
    fn from(s: &HTMLSelectElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLSelectElement);

impl HTMLSelectElement {
    /// Getter of the `autocomplete` attribute.
    /// [`HTMLSelectElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)
    pub fn autocomplete(&self) -> JsString {
        self.inner.get("autocomplete").as_::<JsString>()
    }

    /// Setter of the `autocomplete` attribute.
    /// [`HTMLSelectElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)
    pub fn set_autocomplete(&mut self, value: &JsString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLSelectElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLSelectElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `form` attribute.
    /// [`HTMLSelectElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `multiple` attribute.
    /// [`HTMLSelectElement.multiple`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    /// Setter of the `multiple` attribute.
    /// [`HTMLSelectElement.multiple`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)
    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `name` attribute.
    /// [`HTMLSelectElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLSelectElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `required` attribute.
    /// [`HTMLSelectElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    /// Setter of the `required` attribute.
    /// [`HTMLSelectElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)
    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `size` attribute.
    /// [`HTMLSelectElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

    /// Setter of the `size` attribute.
    /// [`HTMLSelectElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)
    pub fn set_size(&mut self, value: u32) {
        self.inner.set("size", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `type` attribute.
    /// [`HTMLSelectElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `options` attribute.
    /// [`HTMLSelectElement.options`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)
    pub fn options(&self) -> HTMLOptionsCollection {
        self.inner.get("options").as_::<HTMLOptionsCollection>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `length` attribute.
    /// [`HTMLSelectElement.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    /// [`HTMLSelectElement.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `selectedOptions` attribute.
    /// [`HTMLSelectElement.selectedOptions`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)
    pub fn selected_options(&self) -> HTMLCollection {
        self.inner.get("selectedOptions").as_::<HTMLCollection>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `selectedIndex` attribute.
    /// [`HTMLSelectElement.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }

    /// Setter of the `selectedIndex` attribute.
    /// [`HTMLSelectElement.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)
    pub fn set_selected_index(&mut self, value: i32) {
        self.inner.set("selectedIndex", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `value` attribute.
    /// [`HTMLSelectElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLSelectElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl HTMLSelectElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLSelectElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLSelectElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLSelectElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }
}
impl HTMLSelectElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLSelectElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}

impl HTMLSelectElement {
    /// The `new HTMLSelectElement(..)` constructor, creating a new HTMLSelectElement instance
    pub fn new() -> HTMLSelectElement {
        Self {
            inner: Any::global("HTMLSelectElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}

impl HTMLSelectElement {
    /// The item method.
    /// [`HTMLSelectElement.item`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)
    pub fn item(&self, index: u32) -> HTMLOptionElement {
        self.inner
            .call("item", &[index.into()])
            .as_::<HTMLOptionElement>()
    }
}
impl HTMLSelectElement {
    /// The namedItem method.
    /// [`HTMLSelectElement.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)
    pub fn named_item(&self, name: &JsString) -> HTMLOptionElement {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<HTMLOptionElement>()
    }
}
impl HTMLSelectElement {
    /// The add method.
    /// [`HTMLSelectElement.add`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    pub fn add(&self, element: &Any) -> Undefined {
        self.inner.call("add", &[element.into()]).as_::<Undefined>()
    }
}
impl HTMLSelectElement {
    /// The add method.
    /// [`HTMLSelectElement.add`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    pub fn add_with_before(&self, element: &Any, before: &Any) -> Undefined {
        self.inner
            .call("add", &[element.into(), before.into()])
            .as_::<Undefined>()
    }
}
impl HTMLSelectElement {
    /// The remove method.
    /// [`HTMLSelectElement.remove`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)
    pub fn remove(&self) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
impl HTMLSelectElement {
    /// The remove method.
    /// [`HTMLSelectElement.remove`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)
    pub fn remove_with_index(&self, index: i32) -> Undefined {
        self.inner
            .call("remove", &[index.into()])
            .as_::<Undefined>()
    }
}
impl HTMLSelectElement {
    /// The checkValidity method.
    /// [`HTMLSelectElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLSelectElement {
    /// The reportValidity method.
    /// [`HTMLSelectElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLSelectElement {
    /// The setCustomValidity method.
    /// [`HTMLSelectElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &JsString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLSelectElement {
    /// The showPicker method.
    /// [`HTMLSelectElement.showPicker`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/showPicker)
    pub fn show_picker(&self) -> Undefined {
        self.inner.call("showPicker", &[]).as_::<Undefined>()
    }
}
