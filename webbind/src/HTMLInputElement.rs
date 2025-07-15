use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLInputElement {
    inner: HTMLElement,
}
impl FromVal for HTMLInputElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLInputElement {
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
impl core::ops::Deref for HTMLInputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLInputElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLInputElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLInputElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLInputElement> for emlite::Val {
    fn from(s: HTMLInputElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLInputElement> for emlite::Val {
    fn from(s: &HTMLInputElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLInputElement);

impl HTMLInputElement {
    pub fn new() -> HTMLInputElement {
        Self {
            inner: emlite::Val::global("HTMLInputElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLInputElement {
    pub fn accept(&self) -> DOMString {
        self.inner.get("accept").as_::<DOMString>()
    }

    pub fn set_accept(&mut self, value: DOMString) {
        self.inner.set("accept", value);
    }
}
impl HTMLInputElement {
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
impl HTMLInputElement {
    pub fn alt(&self) -> DOMString {
        self.inner.get("alt").as_::<DOMString>()
    }

    pub fn set_alt(&mut self, value: DOMString) {
        self.inner.set("alt", value);
    }
}
impl HTMLInputElement {
    pub fn autocomplete(&self) -> DOMString {
        self.inner.get("autocomplete").as_::<DOMString>()
    }

    pub fn set_autocomplete(&mut self, value: DOMString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLInputElement {
    pub fn default_checked(&self) -> bool {
        self.inner.get("defaultChecked").as_::<bool>()
    }

    pub fn set_default_checked(&mut self, value: bool) {
        self.inner.set("defaultChecked", value);
    }
}
impl HTMLInputElement {
    pub fn checked(&self) -> bool {
        self.inner.get("checked").as_::<bool>()
    }

    pub fn set_checked(&mut self, value: bool) {
        self.inner.set("checked", value);
    }
}
impl HTMLInputElement {
    pub fn color_space(&self) -> DOMString {
        self.inner.get("colorSpace").as_::<DOMString>()
    }

    pub fn set_color_space(&mut self, value: DOMString) {
        self.inner.set("colorSpace", value);
    }
}
impl HTMLInputElement {
    pub fn dir_name(&self) -> DOMString {
        self.inner.get("dirName").as_::<DOMString>()
    }

    pub fn set_dir_name(&mut self, value: DOMString) {
        self.inner.set("dirName", value);
    }
}
impl HTMLInputElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLInputElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLInputElement {
    pub fn files(&self) -> FileList {
        self.inner.get("files").as_::<FileList>()
    }

    pub fn set_files(&mut self, value: FileList) {
        self.inner.set("files", value);
    }
}
impl HTMLInputElement {
    pub fn form_action(&self) -> USVString {
        self.inner.get("formAction").as_::<USVString>()
    }

    pub fn set_form_action(&mut self, value: USVString) {
        self.inner.set("formAction", value);
    }
}
impl HTMLInputElement {
    pub fn form_enctype(&self) -> DOMString {
        self.inner.get("formEnctype").as_::<DOMString>()
    }

    pub fn set_form_enctype(&mut self, value: DOMString) {
        self.inner.set("formEnctype", value);
    }
}
impl HTMLInputElement {
    pub fn form_method(&self) -> DOMString {
        self.inner.get("formMethod").as_::<DOMString>()
    }

    pub fn set_form_method(&mut self, value: DOMString) {
        self.inner.set("formMethod", value);
    }
}
impl HTMLInputElement {
    pub fn form_no_validate(&self) -> bool {
        self.inner.get("formNoValidate").as_::<bool>()
    }

    pub fn set_form_no_validate(&mut self, value: bool) {
        self.inner.set("formNoValidate", value);
    }
}
impl HTMLInputElement {
    pub fn form_target(&self) -> DOMString {
        self.inner.get("formTarget").as_::<DOMString>()
    }

    pub fn set_form_target(&mut self, value: DOMString) {
        self.inner.set("formTarget", value);
    }
}
impl HTMLInputElement {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLInputElement {
    pub fn indeterminate(&self) -> bool {
        self.inner.get("indeterminate").as_::<bool>()
    }

    pub fn set_indeterminate(&mut self, value: bool) {
        self.inner.set("indeterminate", value);
    }
}
impl HTMLInputElement {
    pub fn list(&self) -> HTMLDataListElement {
        self.inner.get("list").as_::<HTMLDataListElement>()
    }
}
impl HTMLInputElement {
    pub fn max(&self) -> DOMString {
        self.inner.get("max").as_::<DOMString>()
    }

    pub fn set_max(&mut self, value: DOMString) {
        self.inner.set("max", value);
    }
}
impl HTMLInputElement {
    pub fn max_length(&self) -> i32 {
        self.inner.get("maxLength").as_::<i32>()
    }

    pub fn set_max_length(&mut self, value: i32) {
        self.inner.set("maxLength", value);
    }
}
impl HTMLInputElement {
    pub fn min(&self) -> DOMString {
        self.inner.get("min").as_::<DOMString>()
    }

    pub fn set_min(&mut self, value: DOMString) {
        self.inner.set("min", value);
    }
}
impl HTMLInputElement {
    pub fn min_length(&self) -> i32 {
        self.inner.get("minLength").as_::<i32>()
    }

    pub fn set_min_length(&mut self, value: i32) {
        self.inner.set("minLength", value);
    }
}
impl HTMLInputElement {
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
impl HTMLInputElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLInputElement {
    pub fn pattern(&self) -> DOMString {
        self.inner.get("pattern").as_::<DOMString>()
    }

    pub fn set_pattern(&mut self, value: DOMString) {
        self.inner.set("pattern", value);
    }
}
impl HTMLInputElement {
    pub fn placeholder(&self) -> DOMString {
        self.inner.get("placeholder").as_::<DOMString>()
    }

    pub fn set_placeholder(&mut self, value: DOMString) {
        self.inner.set("placeholder", value);
    }
}
impl HTMLInputElement {
    pub fn read_only(&self) -> bool {
        self.inner.get("readOnly").as_::<bool>()
    }

    pub fn set_read_only(&mut self, value: bool) {
        self.inner.set("readOnly", value);
    }
}
impl HTMLInputElement {
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLInputElement {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

    pub fn set_size(&mut self, value: u32) {
        self.inner.set("size", value);
    }
}
impl HTMLInputElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLInputElement {
    pub fn step(&self) -> DOMString {
        self.inner.get("step").as_::<DOMString>()
    }

    pub fn set_step(&mut self, value: DOMString) {
        self.inner.set("step", value);
    }
}
impl HTMLInputElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLInputElement {
    pub fn default_value(&self) -> DOMString {
        self.inner.get("defaultValue").as_::<DOMString>()
    }

    pub fn set_default_value(&mut self, value: DOMString) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLInputElement {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLInputElement {
    pub fn value_as_date(&self) -> Object {
        self.inner.get("valueAsDate").as_::<Object>()
    }

    pub fn set_value_as_date(&mut self, value: Object) {
        self.inner.set("valueAsDate", value);
    }
}
impl HTMLInputElement {
    pub fn value_as_number(&self) -> f64 {
        self.inner.get("valueAsNumber").as_::<f64>()
    }

    pub fn set_value_as_number(&mut self, value: f64) {
        self.inner.set("valueAsNumber", value);
    }
}
impl HTMLInputElement {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLInputElement {
    pub fn step_up0(&self) -> Undefined {
        self.inner.call("stepUp", &[]).as_::<Undefined>()
    }

    pub fn step_up1(&self, n: i32) -> Undefined {
        self.inner.call("stepUp", &[n.into()]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    pub fn step_down0(&self) -> Undefined {
        self.inner.call("stepDown", &[]).as_::<Undefined>()
    }

    pub fn step_down1(&self, n: i32) -> Undefined {
        self.inner.call("stepDown", &[n.into()]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLInputElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLInputElement {
    pub fn validation_message(&self) -> DOMString {
        self.inner.get("validationMessage").as_::<DOMString>()
    }
}
impl HTMLInputElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLInputElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLInputElement {
    pub fn set_custom_validity(&self, error: DOMString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLInputElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLInputElement {
    pub fn select(&self) -> Undefined {
        self.inner.call("select", &[]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl HTMLInputElement {
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
impl HTMLInputElement {
    pub fn selection_direction(&self) -> DOMString {
        self.inner.get("selectionDirection").as_::<DOMString>()
    }

    pub fn set_selection_direction(&mut self, value: DOMString) {
        self.inner.set("selectionDirection", value);
    }
}
impl HTMLInputElement {
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
impl HTMLInputElement {
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
impl HTMLInputElement {
    pub fn show_picker(&self) -> Undefined {
        self.inner.call("showPicker", &[]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    pub fn webkitdirectory(&self) -> bool {
        self.inner.get("webkitdirectory").as_::<bool>()
    }

    pub fn set_webkitdirectory(&mut self, value: bool) {
        self.inner.set("webkitdirectory", value);
    }
}
impl HTMLInputElement {
    pub fn webkit_entries(&self) -> FrozenArray<FileSystemEntry> {
        self.inner
            .get("webkitEntries")
            .as_::<FrozenArray<FileSystemEntry>>()
    }
}
impl HTMLInputElement {
    pub fn capture(&self) -> DOMString {
        self.inner.get("capture").as_::<DOMString>()
    }

    pub fn set_capture(&mut self, value: DOMString) {
        self.inner.set("capture", value);
    }
}
impl HTMLInputElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLInputElement {
    pub fn use_map(&self) -> DOMString {
        self.inner.get("useMap").as_::<DOMString>()
    }

    pub fn set_use_map(&mut self, value: DOMString) {
        self.inner.set("useMap", value);
    }
}
impl HTMLInputElement {
    pub fn popover_target_element(&self) -> Element {
        self.inner.get("popoverTargetElement").as_::<Element>()
    }

    pub fn set_popover_target_element(&mut self, value: Element) {
        self.inner.set("popoverTargetElement", value);
    }
}
impl HTMLInputElement {
    pub fn popover_target_action(&self) -> DOMString {
        self.inner.get("popoverTargetAction").as_::<DOMString>()
    }

    pub fn set_popover_target_action(&mut self, value: DOMString) {
        self.inner.set("popoverTargetAction", value);
    }
}
