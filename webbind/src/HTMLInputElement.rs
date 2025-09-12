use super::*;

/// The HTMLInputElement class.
/// [`HTMLInputElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLInputElement {
    inner: HTMLElement,
}

impl FromVal for HTMLInputElement {
    fn from_val(v: &Any) -> Self {
        HTMLInputElement {
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

impl AsRef<Any> for HTMLInputElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLInputElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLInputElement> for Any {
    fn from(s: HTMLInputElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLInputElement> for Any {
    fn from(s: &HTMLInputElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLInputElement);

impl HTMLInputElement {
    /// Getter of the `accept` attribute.
    /// [`HTMLInputElement.accept`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)
    pub fn accept(&self) -> JsString {
        self.inner.get("accept").as_::<JsString>()
    }

    /// Setter of the `accept` attribute.
    /// [`HTMLInputElement.accept`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)
    pub fn set_accept(&mut self, value: &JsString) {
        self.inner.set("accept", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `alpha` attribute.
    /// [`HTMLInputElement.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alpha)
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    /// Setter of the `alpha` attribute.
    /// [`HTMLInputElement.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alpha)
    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `alt` attribute.
    /// [`HTMLInputElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)
    pub fn alt(&self) -> JsString {
        self.inner.get("alt").as_::<JsString>()
    }

    /// Setter of the `alt` attribute.
    /// [`HTMLInputElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)
    pub fn set_alt(&mut self, value: &JsString) {
        self.inner.set("alt", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `autocomplete` attribute.
    /// [`HTMLInputElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)
    pub fn autocomplete(&self) -> JsString {
        self.inner.get("autocomplete").as_::<JsString>()
    }

    /// Setter of the `autocomplete` attribute.
    /// [`HTMLInputElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)
    pub fn set_autocomplete(&mut self, value: &JsString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `defaultChecked` attribute.
    /// [`HTMLInputElement.defaultChecked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)
    pub fn default_checked(&self) -> bool {
        self.inner.get("defaultChecked").as_::<bool>()
    }

    /// Setter of the `defaultChecked` attribute.
    /// [`HTMLInputElement.defaultChecked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)
    pub fn set_default_checked(&mut self, value: bool) {
        self.inner.set("defaultChecked", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `checked` attribute.
    /// [`HTMLInputElement.checked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)
    pub fn checked(&self) -> bool {
        self.inner.get("checked").as_::<bool>()
    }

    /// Setter of the `checked` attribute.
    /// [`HTMLInputElement.checked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)
    pub fn set_checked(&mut self, value: bool) {
        self.inner.set("checked", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `colorSpace` attribute.
    /// [`HTMLInputElement.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/colorSpace)
    pub fn color_space(&self) -> JsString {
        self.inner.get("colorSpace").as_::<JsString>()
    }

    /// Setter of the `colorSpace` attribute.
    /// [`HTMLInputElement.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/colorSpace)
    pub fn set_color_space(&mut self, value: &JsString) {
        self.inner.set("colorSpace", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `dirName` attribute.
    /// [`HTMLInputElement.dirName`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/dirName)
    pub fn dir_name(&self) -> JsString {
        self.inner.get("dirName").as_::<JsString>()
    }

    /// Setter of the `dirName` attribute.
    /// [`HTMLInputElement.dirName`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/dirName)
    pub fn set_dir_name(&mut self, value: &JsString) {
        self.inner.set("dirName", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLInputElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLInputElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `form` attribute.
    /// [`HTMLInputElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLInputElement {
    /// Getter of the `files` attribute.
    /// [`HTMLInputElement.files`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)
    pub fn files(&self) -> FileList {
        self.inner.get("files").as_::<FileList>()
    }

    /// Setter of the `files` attribute.
    /// [`HTMLInputElement.files`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)
    pub fn set_files(&mut self, value: &FileList) {
        self.inner.set("files", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `formAction` attribute.
    /// [`HTMLInputElement.formAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)
    pub fn form_action(&self) -> JsString {
        self.inner.get("formAction").as_::<JsString>()
    }

    /// Setter of the `formAction` attribute.
    /// [`HTMLInputElement.formAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)
    pub fn set_form_action(&mut self, value: &JsString) {
        self.inner.set("formAction", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `formEnctype` attribute.
    /// [`HTMLInputElement.formEnctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)
    pub fn form_enctype(&self) -> JsString {
        self.inner.get("formEnctype").as_::<JsString>()
    }

    /// Setter of the `formEnctype` attribute.
    /// [`HTMLInputElement.formEnctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)
    pub fn set_form_enctype(&mut self, value: &JsString) {
        self.inner.set("formEnctype", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `formMethod` attribute.
    /// [`HTMLInputElement.formMethod`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)
    pub fn form_method(&self) -> JsString {
        self.inner.get("formMethod").as_::<JsString>()
    }

    /// Setter of the `formMethod` attribute.
    /// [`HTMLInputElement.formMethod`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)
    pub fn set_form_method(&mut self, value: &JsString) {
        self.inner.set("formMethod", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `formNoValidate` attribute.
    /// [`HTMLInputElement.formNoValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)
    pub fn form_no_validate(&self) -> bool {
        self.inner.get("formNoValidate").as_::<bool>()
    }

    /// Setter of the `formNoValidate` attribute.
    /// [`HTMLInputElement.formNoValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)
    pub fn set_form_no_validate(&mut self, value: bool) {
        self.inner.set("formNoValidate", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `formTarget` attribute.
    /// [`HTMLInputElement.formTarget`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)
    pub fn form_target(&self) -> JsString {
        self.inner.get("formTarget").as_::<JsString>()
    }

    /// Setter of the `formTarget` attribute.
    /// [`HTMLInputElement.formTarget`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)
    pub fn set_form_target(&mut self, value: &JsString) {
        self.inner.set("formTarget", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `height` attribute.
    /// [`HTMLInputElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLInputElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `indeterminate` attribute.
    /// [`HTMLInputElement.indeterminate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)
    pub fn indeterminate(&self) -> bool {
        self.inner.get("indeterminate").as_::<bool>()
    }

    /// Setter of the `indeterminate` attribute.
    /// [`HTMLInputElement.indeterminate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)
    pub fn set_indeterminate(&mut self, value: bool) {
        self.inner.set("indeterminate", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `list` attribute.
    /// [`HTMLInputElement.list`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/list)
    pub fn list(&self) -> HTMLDataListElement {
        self.inner.get("list").as_::<HTMLDataListElement>()
    }
}
impl HTMLInputElement {
    /// Getter of the `max` attribute.
    /// [`HTMLInputElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)
    pub fn max(&self) -> JsString {
        self.inner.get("max").as_::<JsString>()
    }

    /// Setter of the `max` attribute.
    /// [`HTMLInputElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)
    pub fn set_max(&mut self, value: &JsString) {
        self.inner.set("max", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `maxLength` attribute.
    /// [`HTMLInputElement.maxLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)
    pub fn max_length(&self) -> i32 {
        self.inner.get("maxLength").as_::<i32>()
    }

    /// Setter of the `maxLength` attribute.
    /// [`HTMLInputElement.maxLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)
    pub fn set_max_length(&mut self, value: i32) {
        self.inner.set("maxLength", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `min` attribute.
    /// [`HTMLInputElement.min`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)
    pub fn min(&self) -> JsString {
        self.inner.get("min").as_::<JsString>()
    }

    /// Setter of the `min` attribute.
    /// [`HTMLInputElement.min`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)
    pub fn set_min(&mut self, value: &JsString) {
        self.inner.set("min", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `minLength` attribute.
    /// [`HTMLInputElement.minLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)
    pub fn min_length(&self) -> i32 {
        self.inner.get("minLength").as_::<i32>()
    }

    /// Setter of the `minLength` attribute.
    /// [`HTMLInputElement.minLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)
    pub fn set_min_length(&mut self, value: i32) {
        self.inner.set("minLength", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `multiple` attribute.
    /// [`HTMLInputElement.multiple`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    /// Setter of the `multiple` attribute.
    /// [`HTMLInputElement.multiple`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)
    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `name` attribute.
    /// [`HTMLInputElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLInputElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `pattern` attribute.
    /// [`HTMLInputElement.pattern`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)
    pub fn pattern(&self) -> JsString {
        self.inner.get("pattern").as_::<JsString>()
    }

    /// Setter of the `pattern` attribute.
    /// [`HTMLInputElement.pattern`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)
    pub fn set_pattern(&mut self, value: &JsString) {
        self.inner.set("pattern", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `placeholder` attribute.
    /// [`HTMLInputElement.placeholder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)
    pub fn placeholder(&self) -> JsString {
        self.inner.get("placeholder").as_::<JsString>()
    }

    /// Setter of the `placeholder` attribute.
    /// [`HTMLInputElement.placeholder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)
    pub fn set_placeholder(&mut self, value: &JsString) {
        self.inner.set("placeholder", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `readOnly` attribute.
    /// [`HTMLInputElement.readOnly`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)
    pub fn read_only(&self) -> bool {
        self.inner.get("readOnly").as_::<bool>()
    }

    /// Setter of the `readOnly` attribute.
    /// [`HTMLInputElement.readOnly`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)
    pub fn set_read_only(&mut self, value: bool) {
        self.inner.set("readOnly", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `required` attribute.
    /// [`HTMLInputElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    /// Setter of the `required` attribute.
    /// [`HTMLInputElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)
    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `size` attribute.
    /// [`HTMLInputElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

    /// Setter of the `size` attribute.
    /// [`HTMLInputElement.size`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)
    pub fn set_size(&mut self, value: u32) {
        self.inner.set("size", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `src` attribute.
    /// [`HTMLInputElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLInputElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `step` attribute.
    /// [`HTMLInputElement.step`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)
    pub fn step(&self) -> JsString {
        self.inner.get("step").as_::<JsString>()
    }

    /// Setter of the `step` attribute.
    /// [`HTMLInputElement.step`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)
    pub fn set_step(&mut self, value: &JsString) {
        self.inner.set("step", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `type` attribute.
    /// [`HTMLInputElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLInputElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `defaultValue` attribute.
    /// [`HTMLInputElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)
    pub fn default_value(&self) -> JsString {
        self.inner.get("defaultValue").as_::<JsString>()
    }

    /// Setter of the `defaultValue` attribute.
    /// [`HTMLInputElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)
    pub fn set_default_value(&mut self, value: &JsString) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `value` attribute.
    /// [`HTMLInputElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLInputElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `valueAsDate` attribute.
    /// [`HTMLInputElement.valueAsDate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsDate)
    pub fn value_as_date(&self) -> Object {
        self.inner.get("valueAsDate").as_::<Object>()
    }

    /// Setter of the `valueAsDate` attribute.
    /// [`HTMLInputElement.valueAsDate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsDate)
    pub fn set_value_as_date(&mut self, value: &Object) {
        self.inner.set("valueAsDate", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `valueAsNumber` attribute.
    /// [`HTMLInputElement.valueAsNumber`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)
    pub fn value_as_number(&self) -> f64 {
        self.inner.get("valueAsNumber").as_::<f64>()
    }

    /// Setter of the `valueAsNumber` attribute.
    /// [`HTMLInputElement.valueAsNumber`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)
    pub fn set_value_as_number(&mut self, value: f64) {
        self.inner.set("valueAsNumber", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `width` attribute.
    /// [`HTMLInputElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLInputElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLInputElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLInputElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLInputElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLInputElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLInputElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }
}
impl HTMLInputElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLInputElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLInputElement {
    /// Getter of the `selectionStart` attribute.
    /// [`HTMLInputElement.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    /// Setter of the `selectionStart` attribute.
    /// [`HTMLInputElement.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)
    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `selectionEnd` attribute.
    /// [`HTMLInputElement.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    /// Setter of the `selectionEnd` attribute.
    /// [`HTMLInputElement.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)
    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `selectionDirection` attribute.
    /// [`HTMLInputElement.selectionDirection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)
    pub fn selection_direction(&self) -> JsString {
        self.inner.get("selectionDirection").as_::<JsString>()
    }

    /// Setter of the `selectionDirection` attribute.
    /// [`HTMLInputElement.selectionDirection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)
    pub fn set_selection_direction(&mut self, value: &JsString) {
        self.inner.set("selectionDirection", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `webkitdirectory` attribute.
    /// [`HTMLInputElement.webkitdirectory`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)
    pub fn webkitdirectory(&self) -> bool {
        self.inner.get("webkitdirectory").as_::<bool>()
    }

    /// Setter of the `webkitdirectory` attribute.
    /// [`HTMLInputElement.webkitdirectory`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)
    pub fn set_webkitdirectory(&mut self, value: bool) {
        self.inner.set("webkitdirectory", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `webkitEntries` attribute.
    /// [`HTMLInputElement.webkitEntries`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitEntries)
    pub fn webkit_entries(&self) -> TypedArray<FileSystemEntry> {
        self.inner
            .get("webkitEntries")
            .as_::<TypedArray<FileSystemEntry>>()
    }
}
impl HTMLInputElement {
    /// Getter of the `capture` attribute.
    /// [`HTMLInputElement.capture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/capture)
    pub fn capture(&self) -> JsString {
        self.inner.get("capture").as_::<JsString>()
    }

    /// Setter of the `capture` attribute.
    /// [`HTMLInputElement.capture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/capture)
    pub fn set_capture(&mut self, value: &JsString) {
        self.inner.set("capture", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `align` attribute.
    /// [`HTMLInputElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLInputElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `useMap` attribute.
    /// [`HTMLInputElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)
    pub fn use_map(&self) -> JsString {
        self.inner.get("useMap").as_::<JsString>()
    }

    /// Setter of the `useMap` attribute.
    /// [`HTMLInputElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)
    pub fn set_use_map(&mut self, value: &JsString) {
        self.inner.set("useMap", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `popoverTargetElement` attribute.
    /// [`HTMLInputElement.popoverTargetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetElement)
    pub fn popover_target_element(&self) -> Element {
        self.inner.get("popoverTargetElement").as_::<Element>()
    }

    /// Setter of the `popoverTargetElement` attribute.
    /// [`HTMLInputElement.popoverTargetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetElement)
    pub fn set_popover_target_element(&mut self, value: &Element) {
        self.inner.set("popoverTargetElement", value);
    }
}
impl HTMLInputElement {
    /// Getter of the `popoverTargetAction` attribute.
    /// [`HTMLInputElement.popoverTargetAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetAction)
    pub fn popover_target_action(&self) -> JsString {
        self.inner.get("popoverTargetAction").as_::<JsString>()
    }

    /// Setter of the `popoverTargetAction` attribute.
    /// [`HTMLInputElement.popoverTargetAction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/popoverTargetAction)
    pub fn set_popover_target_action(&mut self, value: &JsString) {
        self.inner.set("popoverTargetAction", value);
    }
}

impl HTMLInputElement {
    /// The `new HTMLInputElement(..)` constructor, creating a new HTMLInputElement instance
    pub fn new() -> HTMLInputElement {
        Self {
            inner: Any::global("HTMLInputElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLInputElement {
    /// The stepUp method.
    /// [`HTMLInputElement.stepUp`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepUp)
    pub fn step_up0(&self) -> Undefined {
        self.inner.call("stepUp", &[]).as_::<Undefined>()
    }
    /// The stepUp method.
    /// [`HTMLInputElement.stepUp`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepUp)
    pub fn step_up1(&self, n: i32) -> Undefined {
        self.inner.call("stepUp", &[n.into()]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The stepDown method.
    /// [`HTMLInputElement.stepDown`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepDown)
    pub fn step_down0(&self) -> Undefined {
        self.inner.call("stepDown", &[]).as_::<Undefined>()
    }
    /// The stepDown method.
    /// [`HTMLInputElement.stepDown`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/stepDown)
    pub fn step_down1(&self, n: i32) -> Undefined {
        self.inner.call("stepDown", &[n.into()]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The checkValidity method.
    /// [`HTMLInputElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLInputElement {
    /// The reportValidity method.
    /// [`HTMLInputElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLInputElement {
    /// The setCustomValidity method.
    /// [`HTMLInputElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &JsString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The select method.
    /// [`HTMLInputElement.select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select)
    pub fn select(&self) -> Undefined {
        self.inner.call("select", &[]).as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The setRangeText method.
    /// [`HTMLInputElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)
    pub fn set_range_text(&self, replacement: &JsString) -> Undefined {
        self.inner
            .call("setRangeText", &[replacement.into()])
            .as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The setRangeText method.
    /// [`HTMLInputElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)
    pub fn set_range_text1(&self, replacement: &JsString, start: u32, end: u32) -> Undefined {
        self.inner
            .call(
                "setRangeText",
                &[replacement.into(), start.into(), end.into()],
            )
            .as_::<Undefined>()
    }
    /// The setRangeText method.
    /// [`HTMLInputElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)
    pub fn set_range_text2(
        &self,
        replacement: &JsString,
        start: u32,
        end: u32,
        selection_mode: &SelectionMode,
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
    /// The setSelectionRange method.
    /// [`HTMLInputElement.setSelectionRange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)
    pub fn set_selection_range0(&self, start: u32, end: u32) -> Undefined {
        self.inner
            .call("setSelectionRange", &[start.into(), end.into()])
            .as_::<Undefined>()
    }
    /// The setSelectionRange method.
    /// [`HTMLInputElement.setSelectionRange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)
    pub fn set_selection_range1(&self, start: u32, end: u32, direction: &JsString) -> Undefined {
        self.inner
            .call(
                "setSelectionRange",
                &[start.into(), end.into(), direction.into()],
            )
            .as_::<Undefined>()
    }
}
impl HTMLInputElement {
    /// The showPicker method.
    /// [`HTMLInputElement.showPicker`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/showPicker)
    pub fn show_picker(&self) -> Undefined {
        self.inner.call("showPicker", &[]).as_::<Undefined>()
    }
}
