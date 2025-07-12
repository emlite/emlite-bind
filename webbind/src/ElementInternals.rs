use super::*;

#[derive(Clone, Debug)]
pub struct ValidityStateFlags {
    inner: emlite::Val,
}
impl FromVal for ValidityStateFlags {
    fn from_val(v: &emlite::Val) -> Self {
        ValidityStateFlags { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ValidityStateFlags {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ValidityStateFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ValidityStateFlags> for emlite::Val {
    fn from(s: ValidityStateFlags) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ValidityStateFlags {
    pub fn value_missing(&self) -> bool {
        self.inner.get("valueMissing").as_::<bool>()
    }

    pub fn set_value_missing(&mut self, value: bool) {
        self.inner.set("valueMissing", value);
    }
}
impl ValidityStateFlags {
    pub fn type_mismatch(&self) -> bool {
        self.inner.get("typeMismatch").as_::<bool>()
    }

    pub fn set_type_mismatch(&mut self, value: bool) {
        self.inner.set("typeMismatch", value);
    }
}
impl ValidityStateFlags {
    pub fn pattern_mismatch(&self) -> bool {
        self.inner.get("patternMismatch").as_::<bool>()
    }

    pub fn set_pattern_mismatch(&mut self, value: bool) {
        self.inner.set("patternMismatch", value);
    }
}
impl ValidityStateFlags {
    pub fn too_long(&self) -> bool {
        self.inner.get("tooLong").as_::<bool>()
    }

    pub fn set_too_long(&mut self, value: bool) {
        self.inner.set("tooLong", value);
    }
}
impl ValidityStateFlags {
    pub fn too_short(&self) -> bool {
        self.inner.get("tooShort").as_::<bool>()
    }

    pub fn set_too_short(&mut self, value: bool) {
        self.inner.set("tooShort", value);
    }
}
impl ValidityStateFlags {
    pub fn range_underflow(&self) -> bool {
        self.inner.get("rangeUnderflow").as_::<bool>()
    }

    pub fn set_range_underflow(&mut self, value: bool) {
        self.inner.set("rangeUnderflow", value);
    }
}
impl ValidityStateFlags {
    pub fn range_overflow(&self) -> bool {
        self.inner.get("rangeOverflow").as_::<bool>()
    }

    pub fn set_range_overflow(&mut self, value: bool) {
        self.inner.set("rangeOverflow", value);
    }
}
impl ValidityStateFlags {
    pub fn step_mismatch(&self) -> bool {
        self.inner.get("stepMismatch").as_::<bool>()
    }

    pub fn set_step_mismatch(&mut self, value: bool) {
        self.inner.set("stepMismatch", value);
    }
}
impl ValidityStateFlags {
    pub fn bad_input(&self) -> bool {
        self.inner.get("badInput").as_::<bool>()
    }

    pub fn set_bad_input(&mut self, value: bool) {
        self.inner.set("badInput", value);
    }
}
impl ValidityStateFlags {
    pub fn custom_error(&self) -> bool {
        self.inner.get("customError").as_::<bool>()
    }

    pub fn set_custom_error(&mut self, value: bool) {
        self.inner.set("customError", value);
    }
}
#[derive(Clone, Debug)]
pub struct ElementInternals {
    inner: emlite::Val,
}
impl FromVal for ElementInternals {
    fn from_val(v: &emlite::Val) -> Self {
        ElementInternals {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ElementInternals {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ElementInternals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ElementInternals> for emlite::Val {
    fn from(s: ElementInternals) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ElementInternals {
    pub fn shadow_root(&self) -> ShadowRoot {
        self.inner.get("shadowRoot").as_::<ShadowRoot>()
    }
}
impl ElementInternals {
    pub fn set_form_value0(&self, value: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("setFormValue", &[value.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_form_value1(&self, value: jsbind::Any, state: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("setFormValue", &[value.into(), state.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ElementInternals {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl ElementInternals {
    pub fn set_validity0(&self) -> jsbind::Undefined {
        self.inner
            .call("setValidity", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_validity1(&self, flags: ValidityStateFlags) -> jsbind::Undefined {
        self.inner
            .call("setValidity", &[flags.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_validity2(
        &self,
        flags: ValidityStateFlags,
        message: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("setValidity", &[flags.into(), message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_validity3(
        &self,
        flags: ValidityStateFlags,
        message: jsbind::DOMString,
        anchor: HTMLElement,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setValidity",
                &[flags.into(), message.into(), anchor.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl ElementInternals {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl ElementInternals {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl ElementInternals {
    pub fn validation_message(&self) -> jsbind::DOMString {
        self.inner
            .get("validationMessage")
            .as_::<jsbind::DOMString>()
    }
}
impl ElementInternals {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl ElementInternals {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl ElementInternals {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
impl ElementInternals {
    pub fn states(&self) -> CustomStateSet {
        self.inner.get("states").as_::<CustomStateSet>()
    }
}
impl ElementInternals {
    pub fn role(&self) -> jsbind::DOMString {
        self.inner.get("role").as_::<jsbind::DOMString>()
    }

    pub fn set_role(&mut self, value: jsbind::DOMString) {
        self.inner.set("role", value);
    }
}
impl ElementInternals {
    pub fn aria_active_descendant_element(&self) -> Element {
        self.inner
            .get("ariaActiveDescendantElement")
            .as_::<Element>()
    }

    pub fn set_aria_active_descendant_element(&mut self, value: Element) {
        self.inner.set("ariaActiveDescendantElement", value);
    }
}
impl ElementInternals {
    pub fn aria_atomic(&self) -> jsbind::DOMString {
        self.inner.get("ariaAtomic").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_atomic(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaAtomic", value);
    }
}
impl ElementInternals {
    pub fn aria_auto_complete(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaAutoComplete")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_auto_complete(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaAutoComplete", value);
    }
}
impl ElementInternals {
    pub fn aria_braille_label(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaBrailleLabel")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_braille_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBrailleLabel", value);
    }
}
impl ElementInternals {
    pub fn aria_braille_role_description(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaBrailleRoleDescription")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_braille_role_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBrailleRoleDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_busy(&self) -> jsbind::DOMString {
        self.inner.get("ariaBusy").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_busy(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBusy", value);
    }
}
impl ElementInternals {
    pub fn aria_checked(&self) -> jsbind::DOMString {
        self.inner.get("ariaChecked").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_checked(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaChecked", value);
    }
}
impl ElementInternals {
    pub fn aria_col_count(&self) -> jsbind::DOMString {
        self.inner.get("ariaColCount").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_count(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColCount", value);
    }
}
impl ElementInternals {
    pub fn aria_col_index(&self) -> jsbind::DOMString {
        self.inner.get("ariaColIndex").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_index(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColIndex", value);
    }
}
impl ElementInternals {
    pub fn aria_col_index_text(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaColIndexText")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_index_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColIndexText", value);
    }
}
impl ElementInternals {
    pub fn aria_col_span(&self) -> jsbind::DOMString {
        self.inner.get("ariaColSpan").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_span(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColSpan", value);
    }
}
impl ElementInternals {
    pub fn aria_controls_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaControlsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_controls_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaControlsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_current(&self) -> jsbind::DOMString {
        self.inner.get("ariaCurrent").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_current(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaCurrent", value);
    }
}
impl ElementInternals {
    pub fn aria_described_by_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaDescribedByElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_described_by_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaDescribedByElements", value);
    }
}
impl ElementInternals {
    pub fn aria_description(&self) -> jsbind::DOMString {
        self.inner.get("ariaDescription").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_details_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaDetailsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_details_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaDetailsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_disabled(&self) -> jsbind::DOMString {
        self.inner.get("ariaDisabled").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_disabled(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaDisabled", value);
    }
}
impl ElementInternals {
    pub fn aria_error_message_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaErrorMessageElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_error_message_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaErrorMessageElements", value);
    }
}
impl ElementInternals {
    pub fn aria_expanded(&self) -> jsbind::DOMString {
        self.inner.get("ariaExpanded").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_expanded(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaExpanded", value);
    }
}
impl ElementInternals {
    pub fn aria_flow_to_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaFlowToElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_flow_to_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaFlowToElements", value);
    }
}
impl ElementInternals {
    pub fn aria_has_popup(&self) -> jsbind::DOMString {
        self.inner.get("ariaHasPopup").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_has_popup(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaHasPopup", value);
    }
}
impl ElementInternals {
    pub fn aria_hidden(&self) -> jsbind::DOMString {
        self.inner.get("ariaHidden").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_hidden(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaHidden", value);
    }
}
impl ElementInternals {
    pub fn aria_invalid(&self) -> jsbind::DOMString {
        self.inner.get("ariaInvalid").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_invalid(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaInvalid", value);
    }
}
impl ElementInternals {
    pub fn aria_key_shortcuts(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaKeyShortcuts")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_key_shortcuts(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaKeyShortcuts", value);
    }
}
impl ElementInternals {
    pub fn aria_label(&self) -> jsbind::DOMString {
        self.inner.get("ariaLabel").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLabel", value);
    }
}
impl ElementInternals {
    pub fn aria_labelled_by_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaLabelledByElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_labelled_by_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaLabelledByElements", value);
    }
}
impl ElementInternals {
    pub fn aria_level(&self) -> jsbind::DOMString {
        self.inner.get("ariaLevel").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_level(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLevel", value);
    }
}
impl ElementInternals {
    pub fn aria_live(&self) -> jsbind::DOMString {
        self.inner.get("ariaLive").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_live(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLive", value);
    }
}
impl ElementInternals {
    pub fn aria_modal(&self) -> jsbind::DOMString {
        self.inner.get("ariaModal").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_modal(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaModal", value);
    }
}
impl ElementInternals {
    pub fn aria_multi_line(&self) -> jsbind::DOMString {
        self.inner.get("ariaMultiLine").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_multi_line(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaMultiLine", value);
    }
}
impl ElementInternals {
    pub fn aria_multi_selectable(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaMultiSelectable")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_multi_selectable(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaMultiSelectable", value);
    }
}
impl ElementInternals {
    pub fn aria_orientation(&self) -> jsbind::DOMString {
        self.inner.get("ariaOrientation").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_orientation(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaOrientation", value);
    }
}
impl ElementInternals {
    pub fn aria_owns_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaOwnsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_owns_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaOwnsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_placeholder(&self) -> jsbind::DOMString {
        self.inner.get("ariaPlaceholder").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_placeholder(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPlaceholder", value);
    }
}
impl ElementInternals {
    pub fn aria_pos_in_set(&self) -> jsbind::DOMString {
        self.inner.get("ariaPosInSet").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_pos_in_set(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPosInSet", value);
    }
}
impl ElementInternals {
    pub fn aria_pressed(&self) -> jsbind::DOMString {
        self.inner.get("ariaPressed").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_pressed(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPressed", value);
    }
}
impl ElementInternals {
    pub fn aria_read_only(&self) -> jsbind::DOMString {
        self.inner.get("ariaReadOnly").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_read_only(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaReadOnly", value);
    }
}
impl ElementInternals {
    pub fn aria_relevant(&self) -> jsbind::DOMString {
        self.inner.get("ariaRelevant").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_relevant(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRelevant", value);
    }
}
impl ElementInternals {
    pub fn aria_required(&self) -> jsbind::DOMString {
        self.inner.get("ariaRequired").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_required(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRequired", value);
    }
}
impl ElementInternals {
    pub fn aria_role_description(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaRoleDescription")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_role_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRoleDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_row_count(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowCount").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_count(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowCount", value);
    }
}
impl ElementInternals {
    pub fn aria_row_index(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowIndex").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_index(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowIndex", value);
    }
}
impl ElementInternals {
    pub fn aria_row_index_text(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaRowIndexText")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_index_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowIndexText", value);
    }
}
impl ElementInternals {
    pub fn aria_row_span(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowSpan").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_span(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowSpan", value);
    }
}
impl ElementInternals {
    pub fn aria_selected(&self) -> jsbind::DOMString {
        self.inner.get("ariaSelected").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_selected(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSelected", value);
    }
}
impl ElementInternals {
    pub fn aria_set_size(&self) -> jsbind::DOMString {
        self.inner.get("ariaSetSize").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_set_size(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSetSize", value);
    }
}
impl ElementInternals {
    pub fn aria_sort(&self) -> jsbind::DOMString {
        self.inner.get("ariaSort").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_sort(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSort", value);
    }
}
impl ElementInternals {
    pub fn aria_value_max(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueMax").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_max(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueMax", value);
    }
}
impl ElementInternals {
    pub fn aria_value_min(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueMin").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_min(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueMin", value);
    }
}
impl ElementInternals {
    pub fn aria_value_now(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueNow").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_now(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueNow", value);
    }
}
impl ElementInternals {
    pub fn aria_value_text(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueText").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueText", value);
    }
}
