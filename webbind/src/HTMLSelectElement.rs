use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLSelectElement {
    inner: HTMLElement,
}
impl FromVal for HTMLSelectElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLSelectElement {
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
impl From<HTMLSelectElement> for emlite::Val {
    fn from(s: HTMLSelectElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLSelectElement {
    pub fn new() -> HTMLSelectElement {
        Self {
            inner: emlite::Val::global("HTMLSelectElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLSelectElement {
    pub fn autocomplete(&self) -> jsbind::DOMString {
        self.inner.get("autocomplete").as_::<jsbind::DOMString>()
    }

    pub fn set_autocomplete(&mut self, value: jsbind::DOMString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLSelectElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLSelectElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLSelectElement {
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
impl HTMLSelectElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLSelectElement {
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLSelectElement {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

    pub fn set_size(&mut self, value: u32) {
        self.inner.set("size", value);
    }
}
impl HTMLSelectElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
impl HTMLSelectElement {
    pub fn options(&self) -> HTMLOptionsCollection {
        self.inner.get("options").as_::<HTMLOptionsCollection>()
    }
}
impl HTMLSelectElement {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl HTMLSelectElement {
    pub fn item(&self, index: u32) -> HTMLOptionElement {
        self.inner
            .call("item", &[index.into()])
            .as_::<HTMLOptionElement>()
    }
}
impl HTMLSelectElement {
    pub fn named_item(&self, name: jsbind::DOMString) -> HTMLOptionElement {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<HTMLOptionElement>()
    }
}
impl HTMLSelectElement {
    pub fn add0(&self, element: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("add", &[element.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn add1(&self, element: jsbind::Any, before: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("add", &[element.into(), before.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLSelectElement {
    pub fn remove(&self, index: i32) -> jsbind::Undefined {
        self.inner
            .call("remove", &[index.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLSelectElement {
    pub fn selected_options(&self) -> HTMLCollection {
        self.inner.get("selectedOptions").as_::<HTMLCollection>()
    }
}
impl HTMLSelectElement {
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }

    pub fn set_selected_index(&mut self, value: i32) {
        self.inner.set("selectedIndex", value);
    }
}
impl HTMLSelectElement {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLSelectElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLSelectElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLSelectElement {
    pub fn validation_message(&self) -> jsbind::DOMString {
        self.inner
            .get("validationMessage")
            .as_::<jsbind::DOMString>()
    }
}
impl HTMLSelectElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLSelectElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLSelectElement {
    pub fn set_custom_validity(&self, error: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLSelectElement {
    pub fn show_picker(&self) -> jsbind::Undefined {
        self.inner
            .call("showPicker", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLSelectElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
