use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTextAreaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTextAreaElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTextAreaElement {
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
impl core::ops::Deref for HTMLTextAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTextAreaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTextAreaElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTextAreaElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLTextAreaElement> for emlite::Val {
    fn from(s: HTMLTextAreaElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTextAreaElement);

impl HTMLTextAreaElement {
    pub fn new() -> HTMLTextAreaElement {
        Self {
            inner: emlite::Val::global("HTMLTextAreaElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTextAreaElement {
    pub fn autocomplete(&self) -> DOMString {
        self.inner.get("autocomplete").as_::<DOMString>()
    }

    pub fn set_autocomplete(&mut self, value: DOMString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLTextAreaElement {
    pub fn cols(&self) -> u32 {
        self.inner.get("cols").as_::<u32>()
    }

    pub fn set_cols(&mut self, value: u32) {
        self.inner.set("cols", value);
    }
}
impl HTMLTextAreaElement {
    pub fn dir_name(&self) -> DOMString {
        self.inner.get("dirName").as_::<DOMString>()
    }

    pub fn set_dir_name(&mut self, value: DOMString) {
        self.inner.set("dirName", value);
    }
}
impl HTMLTextAreaElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLTextAreaElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLTextAreaElement {
    pub fn max_length(&self) -> i32 {
        self.inner.get("maxLength").as_::<i32>()
    }

    pub fn set_max_length(&mut self, value: i32) {
        self.inner.set("maxLength", value);
    }
}
impl HTMLTextAreaElement {
    pub fn min_length(&self) -> i32 {
        self.inner.get("minLength").as_::<i32>()
    }

    pub fn set_min_length(&mut self, value: i32) {
        self.inner.set("minLength", value);
    }
}
impl HTMLTextAreaElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLTextAreaElement {
    pub fn placeholder(&self) -> DOMString {
        self.inner.get("placeholder").as_::<DOMString>()
    }

    pub fn set_placeholder(&mut self, value: DOMString) {
        self.inner.set("placeholder", value);
    }
}
impl HTMLTextAreaElement {
    pub fn read_only(&self) -> bool {
        self.inner.get("readOnly").as_::<bool>()
    }

    pub fn set_read_only(&mut self, value: bool) {
        self.inner.set("readOnly", value);
    }
}
impl HTMLTextAreaElement {
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLTextAreaElement {
    pub fn rows(&self) -> u32 {
        self.inner.get("rows").as_::<u32>()
    }

    pub fn set_rows(&mut self, value: u32) {
        self.inner.set("rows", value);
    }
}
impl HTMLTextAreaElement {
    pub fn wrap(&self) -> DOMString {
        self.inner.get("wrap").as_::<DOMString>()
    }

    pub fn set_wrap(&mut self, value: DOMString) {
        self.inner.set("wrap", value);
    }
}
impl HTMLTextAreaElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
impl HTMLTextAreaElement {
    pub fn default_value(&self) -> DOMString {
        self.inner.get("defaultValue").as_::<DOMString>()
    }

    pub fn set_default_value(&mut self, value: DOMString) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLTextAreaElement {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLTextAreaElement {
    pub fn text_length(&self) -> u32 {
        self.inner.get("textLength").as_::<u32>()
    }
}
impl HTMLTextAreaElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLTextAreaElement {
    pub fn validation_message(&self) -> DOMString {
        self.inner.get("validationMessage").as_::<DOMString>()
    }
}
impl HTMLTextAreaElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    pub fn set_custom_validity(&self, error: DOMString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLTextAreaElement {
    pub fn select(&self) -> Undefined {
        self.inner.call("select", &[]).as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl HTMLTextAreaElement {
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
impl HTMLTextAreaElement {
    pub fn selection_direction(&self) -> DOMString {
        self.inner.get("selectionDirection").as_::<DOMString>()
    }

    pub fn set_selection_direction(&mut self, value: DOMString) {
        self.inner.set("selectionDirection", value);
    }
}
impl HTMLTextAreaElement {
    pub fn set_range_text0(&self, replacement: DOMString, start: u32, end: u32) -> Undefined {
        self.inner
            .call(
                "setRangeText",
                &[replacement.into(), start.into(), end.into()],
            )
            .as_::<Undefined>()
    }

    pub fn set_range_text1(
        &self,
        replacement: DOMString,
        start: u32,
        end: u32,
        selection_mode: SelectionMode,
    ) -> Undefined {
        self.inner
            .call(
                "setRangeText",
                &[
                    replacement.into(),
                    start.into(),
                    end.into(),
                    selection_mode.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    pub fn set_selection_range0(&self, start: u32, end: u32) -> Undefined {
        self.inner
            .call("setSelectionRange", &[start.into(), end.into()])
            .as_::<Undefined>()
    }

    pub fn set_selection_range1(&self, start: u32, end: u32, direction: DOMString) -> Undefined {
        self.inner
            .call(
                "setSelectionRange",
                &[start.into(), end.into(), direction.into()],
            )
            .as_::<Undefined>()
    }
}
