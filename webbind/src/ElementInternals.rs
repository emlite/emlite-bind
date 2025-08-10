use super::*;

/// The ElementInternals class.
/// [`ElementInternals`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ElementInternals {
    inner: Any,
}

impl FromVal for ElementInternals {
    fn from_val(v: &Any) -> Self {
        ElementInternals {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ElementInternals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ElementInternals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ElementInternals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ElementInternals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ElementInternals> for Any {
    fn from(s: ElementInternals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ElementInternals> for Any {
    fn from(s: &ElementInternals) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ElementInternals);

impl ElementInternals {
    /// Getter of the `shadowRoot` attribute.
    /// [`ElementInternals.shadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/shadowRoot)
    pub fn shadow_root(&self) -> ShadowRoot {
        self.inner.get("shadowRoot").as_::<ShadowRoot>()
    }
}
impl ElementInternals {
    /// The setFormValue method.
    /// [`ElementInternals.setFormValue`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setFormValue)
    pub fn set_form_value0(&self, value: &Any) -> Undefined {
        self.inner
            .call("setFormValue", &[value.into()])
            .as_::<Undefined>()
    }
    /// The setFormValue method.
    /// [`ElementInternals.setFormValue`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setFormValue)
    pub fn set_form_value1(&self, value: &Any, state: &Any) -> Undefined {
        self.inner
            .call("setFormValue", &[value.into(), state.into()])
            .as_::<Undefined>()
    }
}
impl ElementInternals {
    /// Getter of the `form` attribute.
    /// [`ElementInternals.form`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl ElementInternals {
    /// The setValidity method.
    /// [`ElementInternals.setValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setValidity)
    pub fn set_validity0(&self) -> Undefined {
        self.inner.call("setValidity", &[]).as_::<Undefined>()
    }
    /// The setValidity method.
    /// [`ElementInternals.setValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setValidity)
    pub fn set_validity1(&self, flags: &ValidityStateFlags) -> Undefined {
        self.inner
            .call("setValidity", &[flags.into()])
            .as_::<Undefined>()
    }
    /// The setValidity method.
    /// [`ElementInternals.setValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setValidity)
    pub fn set_validity2(&self, flags: &ValidityStateFlags, message: &JsString) -> Undefined {
        self.inner
            .call("setValidity", &[flags.into(), message.into()])
            .as_::<Undefined>()
    }
    /// The setValidity method.
    /// [`ElementInternals.setValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/setValidity)
    pub fn set_validity3(
        &self,
        flags: &ValidityStateFlags,
        message: &JsString,
        anchor: &HTMLElement,
    ) -> Undefined {
        self.inner
            .call(
                "setValidity",
                &[flags.into(), message.into(), anchor.into()],
            )
            .as_::<Undefined>()
    }
}
impl ElementInternals {
    /// Getter of the `willValidate` attribute.
    /// [`ElementInternals.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl ElementInternals {
    /// Getter of the `validity` attribute.
    /// [`ElementInternals.validity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl ElementInternals {
    /// Getter of the `validationMessage` attribute.
    /// [`ElementInternals.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }
}
impl ElementInternals {
    /// The checkValidity method.
    /// [`ElementInternals.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl ElementInternals {
    /// The reportValidity method.
    /// [`ElementInternals.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl ElementInternals {
    /// Getter of the `labels` attribute.
    /// [`ElementInternals.labels`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl ElementInternals {
    /// Getter of the `states` attribute.
    /// [`ElementInternals.states`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/states)
    pub fn states(&self) -> CustomStateSet {
        self.inner.get("states").as_::<CustomStateSet>()
    }
}
impl ElementInternals {
    /// Getter of the `role` attribute.
    /// [`ElementInternals.role`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/role)
    pub fn role(&self) -> JsString {
        self.inner.get("role").as_::<JsString>()
    }

    /// Setter of the `role` attribute.
    /// [`ElementInternals.role`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/role)
    pub fn set_role(&mut self, value: &JsString) {
        self.inner.set("role", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaActiveDescendantElement` attribute.
    /// [`ElementInternals.ariaActiveDescendantElement`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaActiveDescendantElement)
    pub fn aria_active_descendant_element(&self) -> Element {
        self.inner
            .get("ariaActiveDescendantElement")
            .as_::<Element>()
    }

    /// Setter of the `ariaActiveDescendantElement` attribute.
    /// [`ElementInternals.ariaActiveDescendantElement`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaActiveDescendantElement)
    pub fn set_aria_active_descendant_element(&mut self, value: &Element) {
        self.inner.set("ariaActiveDescendantElement", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaAtomic` attribute.
    /// [`ElementInternals.ariaAtomic`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaAtomic)
    pub fn aria_atomic(&self) -> JsString {
        self.inner.get("ariaAtomic").as_::<JsString>()
    }

    /// Setter of the `ariaAtomic` attribute.
    /// [`ElementInternals.ariaAtomic`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaAtomic)
    pub fn set_aria_atomic(&mut self, value: &JsString) {
        self.inner.set("ariaAtomic", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaAutoComplete` attribute.
    /// [`ElementInternals.ariaAutoComplete`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaAutoComplete)
    pub fn aria_auto_complete(&self) -> JsString {
        self.inner.get("ariaAutoComplete").as_::<JsString>()
    }

    /// Setter of the `ariaAutoComplete` attribute.
    /// [`ElementInternals.ariaAutoComplete`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaAutoComplete)
    pub fn set_aria_auto_complete(&mut self, value: &JsString) {
        self.inner.set("ariaAutoComplete", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaBrailleLabel` attribute.
    /// [`ElementInternals.ariaBrailleLabel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBrailleLabel)
    pub fn aria_braille_label(&self) -> JsString {
        self.inner.get("ariaBrailleLabel").as_::<JsString>()
    }

    /// Setter of the `ariaBrailleLabel` attribute.
    /// [`ElementInternals.ariaBrailleLabel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBrailleLabel)
    pub fn set_aria_braille_label(&mut self, value: &JsString) {
        self.inner.set("ariaBrailleLabel", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaBrailleRoleDescription` attribute.
    /// [`ElementInternals.ariaBrailleRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBrailleRoleDescription)
    pub fn aria_braille_role_description(&self) -> JsString {
        self.inner
            .get("ariaBrailleRoleDescription")
            .as_::<JsString>()
    }

    /// Setter of the `ariaBrailleRoleDescription` attribute.
    /// [`ElementInternals.ariaBrailleRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBrailleRoleDescription)
    pub fn set_aria_braille_role_description(&mut self, value: &JsString) {
        self.inner.set("ariaBrailleRoleDescription", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaBusy` attribute.
    /// [`ElementInternals.ariaBusy`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBusy)
    pub fn aria_busy(&self) -> JsString {
        self.inner.get("ariaBusy").as_::<JsString>()
    }

    /// Setter of the `ariaBusy` attribute.
    /// [`ElementInternals.ariaBusy`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaBusy)
    pub fn set_aria_busy(&mut self, value: &JsString) {
        self.inner.set("ariaBusy", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaChecked` attribute.
    /// [`ElementInternals.ariaChecked`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaChecked)
    pub fn aria_checked(&self) -> JsString {
        self.inner.get("ariaChecked").as_::<JsString>()
    }

    /// Setter of the `ariaChecked` attribute.
    /// [`ElementInternals.ariaChecked`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaChecked)
    pub fn set_aria_checked(&mut self, value: &JsString) {
        self.inner.set("ariaChecked", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaColCount` attribute.
    /// [`ElementInternals.ariaColCount`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColCount)
    pub fn aria_col_count(&self) -> JsString {
        self.inner.get("ariaColCount").as_::<JsString>()
    }

    /// Setter of the `ariaColCount` attribute.
    /// [`ElementInternals.ariaColCount`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColCount)
    pub fn set_aria_col_count(&mut self, value: &JsString) {
        self.inner.set("ariaColCount", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaColIndex` attribute.
    /// [`ElementInternals.ariaColIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColIndex)
    pub fn aria_col_index(&self) -> JsString {
        self.inner.get("ariaColIndex").as_::<JsString>()
    }

    /// Setter of the `ariaColIndex` attribute.
    /// [`ElementInternals.ariaColIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColIndex)
    pub fn set_aria_col_index(&mut self, value: &JsString) {
        self.inner.set("ariaColIndex", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaColIndexText` attribute.
    /// [`ElementInternals.ariaColIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColIndexText)
    pub fn aria_col_index_text(&self) -> JsString {
        self.inner.get("ariaColIndexText").as_::<JsString>()
    }

    /// Setter of the `ariaColIndexText` attribute.
    /// [`ElementInternals.ariaColIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColIndexText)
    pub fn set_aria_col_index_text(&mut self, value: &JsString) {
        self.inner.set("ariaColIndexText", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaColSpan` attribute.
    /// [`ElementInternals.ariaColSpan`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColSpan)
    pub fn aria_col_span(&self) -> JsString {
        self.inner.get("ariaColSpan").as_::<JsString>()
    }

    /// Setter of the `ariaColSpan` attribute.
    /// [`ElementInternals.ariaColSpan`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaColSpan)
    pub fn set_aria_col_span(&mut self, value: &JsString) {
        self.inner.set("ariaColSpan", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaControlsElements` attribute.
    /// [`ElementInternals.ariaControlsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaControlsElements)
    pub fn aria_controls_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaControlsElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaControlsElements` attribute.
    /// [`ElementInternals.ariaControlsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaControlsElements)
    pub fn set_aria_controls_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaControlsElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaCurrent` attribute.
    /// [`ElementInternals.ariaCurrent`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaCurrent)
    pub fn aria_current(&self) -> JsString {
        self.inner.get("ariaCurrent").as_::<JsString>()
    }

    /// Setter of the `ariaCurrent` attribute.
    /// [`ElementInternals.ariaCurrent`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaCurrent)
    pub fn set_aria_current(&mut self, value: &JsString) {
        self.inner.set("ariaCurrent", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaDescribedByElements` attribute.
    /// [`ElementInternals.ariaDescribedByElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDescribedByElements)
    pub fn aria_described_by_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaDescribedByElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaDescribedByElements` attribute.
    /// [`ElementInternals.ariaDescribedByElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDescribedByElements)
    pub fn set_aria_described_by_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaDescribedByElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaDescription` attribute.
    /// [`ElementInternals.ariaDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDescription)
    pub fn aria_description(&self) -> JsString {
        self.inner.get("ariaDescription").as_::<JsString>()
    }

    /// Setter of the `ariaDescription` attribute.
    /// [`ElementInternals.ariaDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDescription)
    pub fn set_aria_description(&mut self, value: &JsString) {
        self.inner.set("ariaDescription", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaDetailsElements` attribute.
    /// [`ElementInternals.ariaDetailsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDetailsElements)
    pub fn aria_details_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaDetailsElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaDetailsElements` attribute.
    /// [`ElementInternals.ariaDetailsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDetailsElements)
    pub fn set_aria_details_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaDetailsElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaDisabled` attribute.
    /// [`ElementInternals.ariaDisabled`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDisabled)
    pub fn aria_disabled(&self) -> JsString {
        self.inner.get("ariaDisabled").as_::<JsString>()
    }

    /// Setter of the `ariaDisabled` attribute.
    /// [`ElementInternals.ariaDisabled`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaDisabled)
    pub fn set_aria_disabled(&mut self, value: &JsString) {
        self.inner.set("ariaDisabled", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaErrorMessageElements` attribute.
    /// [`ElementInternals.ariaErrorMessageElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaErrorMessageElements)
    pub fn aria_error_message_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaErrorMessageElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaErrorMessageElements` attribute.
    /// [`ElementInternals.ariaErrorMessageElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaErrorMessageElements)
    pub fn set_aria_error_message_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaErrorMessageElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaExpanded` attribute.
    /// [`ElementInternals.ariaExpanded`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaExpanded)
    pub fn aria_expanded(&self) -> JsString {
        self.inner.get("ariaExpanded").as_::<JsString>()
    }

    /// Setter of the `ariaExpanded` attribute.
    /// [`ElementInternals.ariaExpanded`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaExpanded)
    pub fn set_aria_expanded(&mut self, value: &JsString) {
        self.inner.set("ariaExpanded", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaFlowToElements` attribute.
    /// [`ElementInternals.ariaFlowToElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaFlowToElements)
    pub fn aria_flow_to_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaFlowToElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaFlowToElements` attribute.
    /// [`ElementInternals.ariaFlowToElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaFlowToElements)
    pub fn set_aria_flow_to_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaFlowToElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaHasPopup` attribute.
    /// [`ElementInternals.ariaHasPopup`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaHasPopup)
    pub fn aria_has_popup(&self) -> JsString {
        self.inner.get("ariaHasPopup").as_::<JsString>()
    }

    /// Setter of the `ariaHasPopup` attribute.
    /// [`ElementInternals.ariaHasPopup`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaHasPopup)
    pub fn set_aria_has_popup(&mut self, value: &JsString) {
        self.inner.set("ariaHasPopup", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaHidden` attribute.
    /// [`ElementInternals.ariaHidden`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaHidden)
    pub fn aria_hidden(&self) -> JsString {
        self.inner.get("ariaHidden").as_::<JsString>()
    }

    /// Setter of the `ariaHidden` attribute.
    /// [`ElementInternals.ariaHidden`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaHidden)
    pub fn set_aria_hidden(&mut self, value: &JsString) {
        self.inner.set("ariaHidden", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaInvalid` attribute.
    /// [`ElementInternals.ariaInvalid`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaInvalid)
    pub fn aria_invalid(&self) -> JsString {
        self.inner.get("ariaInvalid").as_::<JsString>()
    }

    /// Setter of the `ariaInvalid` attribute.
    /// [`ElementInternals.ariaInvalid`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaInvalid)
    pub fn set_aria_invalid(&mut self, value: &JsString) {
        self.inner.set("ariaInvalid", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaKeyShortcuts` attribute.
    /// [`ElementInternals.ariaKeyShortcuts`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaKeyShortcuts)
    pub fn aria_key_shortcuts(&self) -> JsString {
        self.inner.get("ariaKeyShortcuts").as_::<JsString>()
    }

    /// Setter of the `ariaKeyShortcuts` attribute.
    /// [`ElementInternals.ariaKeyShortcuts`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaKeyShortcuts)
    pub fn set_aria_key_shortcuts(&mut self, value: &JsString) {
        self.inner.set("ariaKeyShortcuts", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaLabel` attribute.
    /// [`ElementInternals.ariaLabel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLabel)
    pub fn aria_label(&self) -> JsString {
        self.inner.get("ariaLabel").as_::<JsString>()
    }

    /// Setter of the `ariaLabel` attribute.
    /// [`ElementInternals.ariaLabel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLabel)
    pub fn set_aria_label(&mut self, value: &JsString) {
        self.inner.set("ariaLabel", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaLabelledByElements` attribute.
    /// [`ElementInternals.ariaLabelledByElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLabelledByElements)
    pub fn aria_labelled_by_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaLabelledByElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaLabelledByElements` attribute.
    /// [`ElementInternals.ariaLabelledByElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLabelledByElements)
    pub fn set_aria_labelled_by_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaLabelledByElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaLevel` attribute.
    /// [`ElementInternals.ariaLevel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLevel)
    pub fn aria_level(&self) -> JsString {
        self.inner.get("ariaLevel").as_::<JsString>()
    }

    /// Setter of the `ariaLevel` attribute.
    /// [`ElementInternals.ariaLevel`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLevel)
    pub fn set_aria_level(&mut self, value: &JsString) {
        self.inner.set("ariaLevel", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaLive` attribute.
    /// [`ElementInternals.ariaLive`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLive)
    pub fn aria_live(&self) -> JsString {
        self.inner.get("ariaLive").as_::<JsString>()
    }

    /// Setter of the `ariaLive` attribute.
    /// [`ElementInternals.ariaLive`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaLive)
    pub fn set_aria_live(&mut self, value: &JsString) {
        self.inner.set("ariaLive", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaModal` attribute.
    /// [`ElementInternals.ariaModal`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaModal)
    pub fn aria_modal(&self) -> JsString {
        self.inner.get("ariaModal").as_::<JsString>()
    }

    /// Setter of the `ariaModal` attribute.
    /// [`ElementInternals.ariaModal`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaModal)
    pub fn set_aria_modal(&mut self, value: &JsString) {
        self.inner.set("ariaModal", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaMultiLine` attribute.
    /// [`ElementInternals.ariaMultiLine`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaMultiLine)
    pub fn aria_multi_line(&self) -> JsString {
        self.inner.get("ariaMultiLine").as_::<JsString>()
    }

    /// Setter of the `ariaMultiLine` attribute.
    /// [`ElementInternals.ariaMultiLine`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaMultiLine)
    pub fn set_aria_multi_line(&mut self, value: &JsString) {
        self.inner.set("ariaMultiLine", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaMultiSelectable` attribute.
    /// [`ElementInternals.ariaMultiSelectable`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaMultiSelectable)
    pub fn aria_multi_selectable(&self) -> JsString {
        self.inner.get("ariaMultiSelectable").as_::<JsString>()
    }

    /// Setter of the `ariaMultiSelectable` attribute.
    /// [`ElementInternals.ariaMultiSelectable`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaMultiSelectable)
    pub fn set_aria_multi_selectable(&mut self, value: &JsString) {
        self.inner.set("ariaMultiSelectable", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaOrientation` attribute.
    /// [`ElementInternals.ariaOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaOrientation)
    pub fn aria_orientation(&self) -> JsString {
        self.inner.get("ariaOrientation").as_::<JsString>()
    }

    /// Setter of the `ariaOrientation` attribute.
    /// [`ElementInternals.ariaOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaOrientation)
    pub fn set_aria_orientation(&mut self, value: &JsString) {
        self.inner.set("ariaOrientation", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaOwnsElements` attribute.
    /// [`ElementInternals.ariaOwnsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaOwnsElements)
    pub fn aria_owns_elements(&self) -> TypedArray<Element> {
        self.inner
            .get("ariaOwnsElements")
            .as_::<TypedArray<Element>>()
    }

    /// Setter of the `ariaOwnsElements` attribute.
    /// [`ElementInternals.ariaOwnsElements`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaOwnsElements)
    pub fn set_aria_owns_elements(&mut self, value: &TypedArray<Element>) {
        self.inner.set("ariaOwnsElements", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaPlaceholder` attribute.
    /// [`ElementInternals.ariaPlaceholder`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPlaceholder)
    pub fn aria_placeholder(&self) -> JsString {
        self.inner.get("ariaPlaceholder").as_::<JsString>()
    }

    /// Setter of the `ariaPlaceholder` attribute.
    /// [`ElementInternals.ariaPlaceholder`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPlaceholder)
    pub fn set_aria_placeholder(&mut self, value: &JsString) {
        self.inner.set("ariaPlaceholder", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaPosInSet` attribute.
    /// [`ElementInternals.ariaPosInSet`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPosInSet)
    pub fn aria_pos_in_set(&self) -> JsString {
        self.inner.get("ariaPosInSet").as_::<JsString>()
    }

    /// Setter of the `ariaPosInSet` attribute.
    /// [`ElementInternals.ariaPosInSet`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPosInSet)
    pub fn set_aria_pos_in_set(&mut self, value: &JsString) {
        self.inner.set("ariaPosInSet", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaPressed` attribute.
    /// [`ElementInternals.ariaPressed`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPressed)
    pub fn aria_pressed(&self) -> JsString {
        self.inner.get("ariaPressed").as_::<JsString>()
    }

    /// Setter of the `ariaPressed` attribute.
    /// [`ElementInternals.ariaPressed`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaPressed)
    pub fn set_aria_pressed(&mut self, value: &JsString) {
        self.inner.set("ariaPressed", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaReadOnly` attribute.
    /// [`ElementInternals.ariaReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaReadOnly)
    pub fn aria_read_only(&self) -> JsString {
        self.inner.get("ariaReadOnly").as_::<JsString>()
    }

    /// Setter of the `ariaReadOnly` attribute.
    /// [`ElementInternals.ariaReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaReadOnly)
    pub fn set_aria_read_only(&mut self, value: &JsString) {
        self.inner.set("ariaReadOnly", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRelevant` attribute.
    /// [`ElementInternals.ariaRelevant`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRelevant)
    pub fn aria_relevant(&self) -> JsString {
        self.inner.get("ariaRelevant").as_::<JsString>()
    }

    /// Setter of the `ariaRelevant` attribute.
    /// [`ElementInternals.ariaRelevant`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRelevant)
    pub fn set_aria_relevant(&mut self, value: &JsString) {
        self.inner.set("ariaRelevant", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRequired` attribute.
    /// [`ElementInternals.ariaRequired`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRequired)
    pub fn aria_required(&self) -> JsString {
        self.inner.get("ariaRequired").as_::<JsString>()
    }

    /// Setter of the `ariaRequired` attribute.
    /// [`ElementInternals.ariaRequired`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRequired)
    pub fn set_aria_required(&mut self, value: &JsString) {
        self.inner.set("ariaRequired", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRoleDescription` attribute.
    /// [`ElementInternals.ariaRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRoleDescription)
    pub fn aria_role_description(&self) -> JsString {
        self.inner.get("ariaRoleDescription").as_::<JsString>()
    }

    /// Setter of the `ariaRoleDescription` attribute.
    /// [`ElementInternals.ariaRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRoleDescription)
    pub fn set_aria_role_description(&mut self, value: &JsString) {
        self.inner.set("ariaRoleDescription", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRowCount` attribute.
    /// [`ElementInternals.ariaRowCount`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowCount)
    pub fn aria_row_count(&self) -> JsString {
        self.inner.get("ariaRowCount").as_::<JsString>()
    }

    /// Setter of the `ariaRowCount` attribute.
    /// [`ElementInternals.ariaRowCount`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowCount)
    pub fn set_aria_row_count(&mut self, value: &JsString) {
        self.inner.set("ariaRowCount", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRowIndex` attribute.
    /// [`ElementInternals.ariaRowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowIndex)
    pub fn aria_row_index(&self) -> JsString {
        self.inner.get("ariaRowIndex").as_::<JsString>()
    }

    /// Setter of the `ariaRowIndex` attribute.
    /// [`ElementInternals.ariaRowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowIndex)
    pub fn set_aria_row_index(&mut self, value: &JsString) {
        self.inner.set("ariaRowIndex", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRowIndexText` attribute.
    /// [`ElementInternals.ariaRowIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowIndexText)
    pub fn aria_row_index_text(&self) -> JsString {
        self.inner.get("ariaRowIndexText").as_::<JsString>()
    }

    /// Setter of the `ariaRowIndexText` attribute.
    /// [`ElementInternals.ariaRowIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowIndexText)
    pub fn set_aria_row_index_text(&mut self, value: &JsString) {
        self.inner.set("ariaRowIndexText", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaRowSpan` attribute.
    /// [`ElementInternals.ariaRowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowSpan)
    pub fn aria_row_span(&self) -> JsString {
        self.inner.get("ariaRowSpan").as_::<JsString>()
    }

    /// Setter of the `ariaRowSpan` attribute.
    /// [`ElementInternals.ariaRowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaRowSpan)
    pub fn set_aria_row_span(&mut self, value: &JsString) {
        self.inner.set("ariaRowSpan", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaSelected` attribute.
    /// [`ElementInternals.ariaSelected`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSelected)
    pub fn aria_selected(&self) -> JsString {
        self.inner.get("ariaSelected").as_::<JsString>()
    }

    /// Setter of the `ariaSelected` attribute.
    /// [`ElementInternals.ariaSelected`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSelected)
    pub fn set_aria_selected(&mut self, value: &JsString) {
        self.inner.set("ariaSelected", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaSetSize` attribute.
    /// [`ElementInternals.ariaSetSize`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSetSize)
    pub fn aria_set_size(&self) -> JsString {
        self.inner.get("ariaSetSize").as_::<JsString>()
    }

    /// Setter of the `ariaSetSize` attribute.
    /// [`ElementInternals.ariaSetSize`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSetSize)
    pub fn set_aria_set_size(&mut self, value: &JsString) {
        self.inner.set("ariaSetSize", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaSort` attribute.
    /// [`ElementInternals.ariaSort`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSort)
    pub fn aria_sort(&self) -> JsString {
        self.inner.get("ariaSort").as_::<JsString>()
    }

    /// Setter of the `ariaSort` attribute.
    /// [`ElementInternals.ariaSort`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaSort)
    pub fn set_aria_sort(&mut self, value: &JsString) {
        self.inner.set("ariaSort", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaValueMax` attribute.
    /// [`ElementInternals.ariaValueMax`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueMax)
    pub fn aria_value_max(&self) -> JsString {
        self.inner.get("ariaValueMax").as_::<JsString>()
    }

    /// Setter of the `ariaValueMax` attribute.
    /// [`ElementInternals.ariaValueMax`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueMax)
    pub fn set_aria_value_max(&mut self, value: &JsString) {
        self.inner.set("ariaValueMax", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaValueMin` attribute.
    /// [`ElementInternals.ariaValueMin`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueMin)
    pub fn aria_value_min(&self) -> JsString {
        self.inner.get("ariaValueMin").as_::<JsString>()
    }

    /// Setter of the `ariaValueMin` attribute.
    /// [`ElementInternals.ariaValueMin`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueMin)
    pub fn set_aria_value_min(&mut self, value: &JsString) {
        self.inner.set("ariaValueMin", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaValueNow` attribute.
    /// [`ElementInternals.ariaValueNow`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueNow)
    pub fn aria_value_now(&self) -> JsString {
        self.inner.get("ariaValueNow").as_::<JsString>()
    }

    /// Setter of the `ariaValueNow` attribute.
    /// [`ElementInternals.ariaValueNow`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueNow)
    pub fn set_aria_value_now(&mut self, value: &JsString) {
        self.inner.set("ariaValueNow", value);
    }
}
impl ElementInternals {
    /// Getter of the `ariaValueText` attribute.
    /// [`ElementInternals.ariaValueText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueText)
    pub fn aria_value_text(&self) -> JsString {
        self.inner.get("ariaValueText").as_::<JsString>()
    }

    /// Setter of the `ariaValueText` attribute.
    /// [`ElementInternals.ariaValueText`](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals/ariaValueText)
    pub fn set_aria_value_text(&mut self, value: &JsString) {
        self.inner.set("ariaValueText", value);
    }
}
