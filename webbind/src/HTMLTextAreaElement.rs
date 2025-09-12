use super::*;

/// The HTMLTextAreaElement class.
/// [`HTMLTextAreaElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTextAreaElement {
    inner: HTMLElement,
}

impl FromVal for HTMLTextAreaElement {
    fn from_val(v: &Any) -> Self {
        HTMLTextAreaElement {
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

impl AsRef<Any> for HTMLTextAreaElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLTextAreaElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLTextAreaElement> for Any {
    fn from(s: HTMLTextAreaElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLTextAreaElement> for Any {
    fn from(s: &HTMLTextAreaElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLTextAreaElement);

impl HTMLTextAreaElement {
    /// Getter of the `autocomplete` attribute.
    /// [`HTMLTextAreaElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)
    pub fn autocomplete(&self) -> JsString {
        self.inner.get("autocomplete").as_::<JsString>()
    }

    /// Setter of the `autocomplete` attribute.
    /// [`HTMLTextAreaElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)
    pub fn set_autocomplete(&mut self, value: &JsString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `cols` attribute.
    /// [`HTMLTextAreaElement.cols`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)
    pub fn cols(&self) -> u32 {
        self.inner.get("cols").as_::<u32>()
    }

    /// Setter of the `cols` attribute.
    /// [`HTMLTextAreaElement.cols`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)
    pub fn set_cols(&mut self, value: u32) {
        self.inner.set("cols", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `dirName` attribute.
    /// [`HTMLTextAreaElement.dirName`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/dirName)
    pub fn dir_name(&self) -> JsString {
        self.inner.get("dirName").as_::<JsString>()
    }

    /// Setter of the `dirName` attribute.
    /// [`HTMLTextAreaElement.dirName`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/dirName)
    pub fn set_dir_name(&mut self, value: &JsString) {
        self.inner.set("dirName", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLTextAreaElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLTextAreaElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `form` attribute.
    /// [`HTMLTextAreaElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `maxLength` attribute.
    /// [`HTMLTextAreaElement.maxLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)
    pub fn max_length(&self) -> i32 {
        self.inner.get("maxLength").as_::<i32>()
    }

    /// Setter of the `maxLength` attribute.
    /// [`HTMLTextAreaElement.maxLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)
    pub fn set_max_length(&mut self, value: i32) {
        self.inner.set("maxLength", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `minLength` attribute.
    /// [`HTMLTextAreaElement.minLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)
    pub fn min_length(&self) -> i32 {
        self.inner.get("minLength").as_::<i32>()
    }

    /// Setter of the `minLength` attribute.
    /// [`HTMLTextAreaElement.minLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)
    pub fn set_min_length(&mut self, value: i32) {
        self.inner.set("minLength", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `name` attribute.
    /// [`HTMLTextAreaElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLTextAreaElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `placeholder` attribute.
    /// [`HTMLTextAreaElement.placeholder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)
    pub fn placeholder(&self) -> JsString {
        self.inner.get("placeholder").as_::<JsString>()
    }

    /// Setter of the `placeholder` attribute.
    /// [`HTMLTextAreaElement.placeholder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)
    pub fn set_placeholder(&mut self, value: &JsString) {
        self.inner.set("placeholder", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `readOnly` attribute.
    /// [`HTMLTextAreaElement.readOnly`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)
    pub fn read_only(&self) -> bool {
        self.inner.get("readOnly").as_::<bool>()
    }

    /// Setter of the `readOnly` attribute.
    /// [`HTMLTextAreaElement.readOnly`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)
    pub fn set_read_only(&mut self, value: bool) {
        self.inner.set("readOnly", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `required` attribute.
    /// [`HTMLTextAreaElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)
    pub fn required(&self) -> bool {
        self.inner.get("required").as_::<bool>()
    }

    /// Setter of the `required` attribute.
    /// [`HTMLTextAreaElement.required`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)
    pub fn set_required(&mut self, value: bool) {
        self.inner.set("required", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `rows` attribute.
    /// [`HTMLTextAreaElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)
    pub fn rows(&self) -> u32 {
        self.inner.get("rows").as_::<u32>()
    }

    /// Setter of the `rows` attribute.
    /// [`HTMLTextAreaElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)
    pub fn set_rows(&mut self, value: u32) {
        self.inner.set("rows", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `wrap` attribute.
    /// [`HTMLTextAreaElement.wrap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)
    pub fn wrap(&self) -> JsString {
        self.inner.get("wrap").as_::<JsString>()
    }

    /// Setter of the `wrap` attribute.
    /// [`HTMLTextAreaElement.wrap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)
    pub fn set_wrap(&mut self, value: &JsString) {
        self.inner.set("wrap", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `type` attribute.
    /// [`HTMLTextAreaElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `defaultValue` attribute.
    /// [`HTMLTextAreaElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)
    pub fn default_value(&self) -> JsString {
        self.inner.get("defaultValue").as_::<JsString>()
    }

    /// Setter of the `defaultValue` attribute.
    /// [`HTMLTextAreaElement.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)
    pub fn set_default_value(&mut self, value: &JsString) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `value` attribute.
    /// [`HTMLTextAreaElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLTextAreaElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `textLength` attribute.
    /// [`HTMLTextAreaElement.textLength`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength)
    pub fn text_length(&self) -> u32 {
        self.inner.get("textLength").as_::<u32>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLTextAreaElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLTextAreaElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLTextAreaElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLTextAreaElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `selectionStart` attribute.
    /// [`HTMLTextAreaElement.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    /// Setter of the `selectionStart` attribute.
    /// [`HTMLTextAreaElement.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)
    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `selectionEnd` attribute.
    /// [`HTMLTextAreaElement.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    /// Setter of the `selectionEnd` attribute.
    /// [`HTMLTextAreaElement.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)
    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
impl HTMLTextAreaElement {
    /// Getter of the `selectionDirection` attribute.
    /// [`HTMLTextAreaElement.selectionDirection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)
    pub fn selection_direction(&self) -> JsString {
        self.inner.get("selectionDirection").as_::<JsString>()
    }

    /// Setter of the `selectionDirection` attribute.
    /// [`HTMLTextAreaElement.selectionDirection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)
    pub fn set_selection_direction(&mut self, value: &JsString) {
        self.inner.set("selectionDirection", value);
    }
}

impl HTMLTextAreaElement {
    /// The `new HTMLTextAreaElement(..)` constructor, creating a new HTMLTextAreaElement instance
    pub fn new() -> HTMLTextAreaElement {
        Self {
            inner: Any::global("HTMLTextAreaElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTextAreaElement {
    /// The checkValidity method.
    /// [`HTMLTextAreaElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    /// The reportValidity method.
    /// [`HTMLTextAreaElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLTextAreaElement {
    /// The setCustomValidity method.
    /// [`HTMLTextAreaElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &JsString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    /// The select method.
    /// [`HTMLTextAreaElement.select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select)
    pub fn select(&self) -> Undefined {
        self.inner.call("select", &[]).as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    /// The setRangeText method.
    /// [`HTMLTextAreaElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)
    pub fn set_range_text(&self, replacement: &JsString) -> Undefined {
        self.inner
            .call("setRangeText", &[replacement.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTextAreaElement {
    /// The setRangeText method.
    /// [`HTMLTextAreaElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)
    pub fn set_range_text1(&self, replacement: &JsString, start: u32, end: u32) -> Undefined {
        self.inner
            .call(
                "setRangeText",
                &[replacement.into(), start.into(), end.into()],
            )
            .as_::<Undefined>()
    }
    /// The setRangeText method.
    /// [`HTMLTextAreaElement.setRangeText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)
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
impl HTMLTextAreaElement {
    /// The setSelectionRange method.
    /// [`HTMLTextAreaElement.setSelectionRange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)
    pub fn set_selection_range0(&self, start: u32, end: u32) -> Undefined {
        self.inner
            .call("setSelectionRange", &[start.into(), end.into()])
            .as_::<Undefined>()
    }
    /// The setSelectionRange method.
    /// [`HTMLTextAreaElement.setSelectionRange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)
    pub fn set_selection_range1(&self, start: u32, end: u32, direction: &JsString) -> Undefined {
        self.inner
            .call(
                "setSelectionRange",
                &[start.into(), end.into(), direction.into()],
            )
            .as_::<Undefined>()
    }
}
