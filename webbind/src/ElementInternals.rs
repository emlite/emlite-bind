use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ValidityStateFlags {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ValidityStateFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ValidityStateFlags {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ValidityStateFlags {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ValidityStateFlags> for emlite::Val {
    fn from(s: ValidityStateFlags) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ValidityStateFlags> for emlite::Val {
    fn from(s: &ValidityStateFlags) -> emlite::Val {
        s.inner.clone()
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ElementInternals {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ElementInternals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ElementInternals {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ElementInternals {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ElementInternals> for emlite::Val {
    fn from(s: ElementInternals) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ElementInternals> for emlite::Val {
    fn from(s: &ElementInternals) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ElementInternals);

impl ElementInternals {
    pub fn shadow_root(&self) -> ShadowRoot {
        self.inner.get("shadowRoot").as_::<ShadowRoot>()
    }
}
impl ElementInternals {
    pub fn set_form_value0(&self, value: &Any) -> Undefined {
        self.inner
            .call("setFormValue", &[value.into()])
            .as_::<Undefined>()
    }

    pub fn set_form_value1(&self, value: &Any, state: &Any) -> Undefined {
        self.inner
            .call("setFormValue", &[value.into(), state.into()])
            .as_::<Undefined>()
    }
}
impl ElementInternals {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl ElementInternals {
    pub fn set_validity0(&self) -> Undefined {
        self.inner.call("setValidity", &[]).as_::<Undefined>()
    }

    pub fn set_validity1(&self, flags: &ValidityStateFlags) -> Undefined {
        self.inner
            .call("setValidity", &[flags.into()])
            .as_::<Undefined>()
    }

    pub fn set_validity2(&self, flags: &ValidityStateFlags, message: &str) -> Undefined {
        self.inner
            .call("setValidity", &[flags.into(), message.into()])
            .as_::<Undefined>()
    }

    pub fn set_validity3(
        &self,
        flags: &ValidityStateFlags,
        message: &str,
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
    pub fn validation_message(&self) -> String {
        self.inner.get("validationMessage").as_::<String>()
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
    pub fn role(&self) -> String {
        self.inner.get("role").as_::<String>()
    }

    pub fn set_role(&mut self, value: &str) {
        self.inner.set("role", value);
    }
}
impl ElementInternals {
    pub fn aria_active_descendant_element(&self) -> Element {
        self.inner
            .get("ariaActiveDescendantElement")
            .as_::<Element>()
    }

    pub fn set_aria_active_descendant_element(&mut self, value: &Element) {
        self.inner.set("ariaActiveDescendantElement", value);
    }
}
impl ElementInternals {
    pub fn aria_atomic(&self) -> String {
        self.inner.get("ariaAtomic").as_::<String>()
    }

    pub fn set_aria_atomic(&mut self, value: &str) {
        self.inner.set("ariaAtomic", value);
    }
}
impl ElementInternals {
    pub fn aria_auto_complete(&self) -> String {
        self.inner.get("ariaAutoComplete").as_::<String>()
    }

    pub fn set_aria_auto_complete(&mut self, value: &str) {
        self.inner.set("ariaAutoComplete", value);
    }
}
impl ElementInternals {
    pub fn aria_braille_label(&self) -> String {
        self.inner.get("ariaBrailleLabel").as_::<String>()
    }

    pub fn set_aria_braille_label(&mut self, value: &str) {
        self.inner.set("ariaBrailleLabel", value);
    }
}
impl ElementInternals {
    pub fn aria_braille_role_description(&self) -> String {
        self.inner.get("ariaBrailleRoleDescription").as_::<String>()
    }

    pub fn set_aria_braille_role_description(&mut self, value: &str) {
        self.inner.set("ariaBrailleRoleDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_busy(&self) -> String {
        self.inner.get("ariaBusy").as_::<String>()
    }

    pub fn set_aria_busy(&mut self, value: &str) {
        self.inner.set("ariaBusy", value);
    }
}
impl ElementInternals {
    pub fn aria_checked(&self) -> String {
        self.inner.get("ariaChecked").as_::<String>()
    }

    pub fn set_aria_checked(&mut self, value: &str) {
        self.inner.set("ariaChecked", value);
    }
}
impl ElementInternals {
    pub fn aria_col_count(&self) -> String {
        self.inner.get("ariaColCount").as_::<String>()
    }

    pub fn set_aria_col_count(&mut self, value: &str) {
        self.inner.set("ariaColCount", value);
    }
}
impl ElementInternals {
    pub fn aria_col_index(&self) -> String {
        self.inner.get("ariaColIndex").as_::<String>()
    }

    pub fn set_aria_col_index(&mut self, value: &str) {
        self.inner.set("ariaColIndex", value);
    }
}
impl ElementInternals {
    pub fn aria_col_index_text(&self) -> String {
        self.inner.get("ariaColIndexText").as_::<String>()
    }

    pub fn set_aria_col_index_text(&mut self, value: &str) {
        self.inner.set("ariaColIndexText", value);
    }
}
impl ElementInternals {
    pub fn aria_col_span(&self) -> String {
        self.inner.get("ariaColSpan").as_::<String>()
    }

    pub fn set_aria_col_span(&mut self, value: &str) {
        self.inner.set("ariaColSpan", value);
    }
}
impl ElementInternals {
    pub fn aria_controls_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaControlsElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_controls_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaControlsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_current(&self) -> String {
        self.inner.get("ariaCurrent").as_::<String>()
    }

    pub fn set_aria_current(&mut self, value: &str) {
        self.inner.set("ariaCurrent", value);
    }
}
impl ElementInternals {
    pub fn aria_described_by_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaDescribedByElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_described_by_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaDescribedByElements", value);
    }
}
impl ElementInternals {
    pub fn aria_description(&self) -> String {
        self.inner.get("ariaDescription").as_::<String>()
    }

    pub fn set_aria_description(&mut self, value: &str) {
        self.inner.set("ariaDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_details_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaDetailsElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_details_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaDetailsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_disabled(&self) -> String {
        self.inner.get("ariaDisabled").as_::<String>()
    }

    pub fn set_aria_disabled(&mut self, value: &str) {
        self.inner.set("ariaDisabled", value);
    }
}
impl ElementInternals {
    pub fn aria_error_message_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaErrorMessageElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_error_message_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaErrorMessageElements", value);
    }
}
impl ElementInternals {
    pub fn aria_expanded(&self) -> String {
        self.inner.get("ariaExpanded").as_::<String>()
    }

    pub fn set_aria_expanded(&mut self, value: &str) {
        self.inner.set("ariaExpanded", value);
    }
}
impl ElementInternals {
    pub fn aria_flow_to_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaFlowToElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_flow_to_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaFlowToElements", value);
    }
}
impl ElementInternals {
    pub fn aria_has_popup(&self) -> String {
        self.inner.get("ariaHasPopup").as_::<String>()
    }

    pub fn set_aria_has_popup(&mut self, value: &str) {
        self.inner.set("ariaHasPopup", value);
    }
}
impl ElementInternals {
    pub fn aria_hidden(&self) -> String {
        self.inner.get("ariaHidden").as_::<String>()
    }

    pub fn set_aria_hidden(&mut self, value: &str) {
        self.inner.set("ariaHidden", value);
    }
}
impl ElementInternals {
    pub fn aria_invalid(&self) -> String {
        self.inner.get("ariaInvalid").as_::<String>()
    }

    pub fn set_aria_invalid(&mut self, value: &str) {
        self.inner.set("ariaInvalid", value);
    }
}
impl ElementInternals {
    pub fn aria_key_shortcuts(&self) -> String {
        self.inner.get("ariaKeyShortcuts").as_::<String>()
    }

    pub fn set_aria_key_shortcuts(&mut self, value: &str) {
        self.inner.set("ariaKeyShortcuts", value);
    }
}
impl ElementInternals {
    pub fn aria_label(&self) -> String {
        self.inner.get("ariaLabel").as_::<String>()
    }

    pub fn set_aria_label(&mut self, value: &str) {
        self.inner.set("ariaLabel", value);
    }
}
impl ElementInternals {
    pub fn aria_labelled_by_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaLabelledByElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_labelled_by_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaLabelledByElements", value);
    }
}
impl ElementInternals {
    pub fn aria_level(&self) -> String {
        self.inner.get("ariaLevel").as_::<String>()
    }

    pub fn set_aria_level(&mut self, value: &str) {
        self.inner.set("ariaLevel", value);
    }
}
impl ElementInternals {
    pub fn aria_live(&self) -> String {
        self.inner.get("ariaLive").as_::<String>()
    }

    pub fn set_aria_live(&mut self, value: &str) {
        self.inner.set("ariaLive", value);
    }
}
impl ElementInternals {
    pub fn aria_modal(&self) -> String {
        self.inner.get("ariaModal").as_::<String>()
    }

    pub fn set_aria_modal(&mut self, value: &str) {
        self.inner.set("ariaModal", value);
    }
}
impl ElementInternals {
    pub fn aria_multi_line(&self) -> String {
        self.inner.get("ariaMultiLine").as_::<String>()
    }

    pub fn set_aria_multi_line(&mut self, value: &str) {
        self.inner.set("ariaMultiLine", value);
    }
}
impl ElementInternals {
    pub fn aria_multi_selectable(&self) -> String {
        self.inner.get("ariaMultiSelectable").as_::<String>()
    }

    pub fn set_aria_multi_selectable(&mut self, value: &str) {
        self.inner.set("ariaMultiSelectable", value);
    }
}
impl ElementInternals {
    pub fn aria_orientation(&self) -> String {
        self.inner.get("ariaOrientation").as_::<String>()
    }

    pub fn set_aria_orientation(&mut self, value: &str) {
        self.inner.set("ariaOrientation", value);
    }
}
impl ElementInternals {
    pub fn aria_owns_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaOwnsElements")
            .as_::<FrozenArray<Element>>()
    }

    pub fn set_aria_owns_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaOwnsElements", value);
    }
}
impl ElementInternals {
    pub fn aria_placeholder(&self) -> String {
        self.inner.get("ariaPlaceholder").as_::<String>()
    }

    pub fn set_aria_placeholder(&mut self, value: &str) {
        self.inner.set("ariaPlaceholder", value);
    }
}
impl ElementInternals {
    pub fn aria_pos_in_set(&self) -> String {
        self.inner.get("ariaPosInSet").as_::<String>()
    }

    pub fn set_aria_pos_in_set(&mut self, value: &str) {
        self.inner.set("ariaPosInSet", value);
    }
}
impl ElementInternals {
    pub fn aria_pressed(&self) -> String {
        self.inner.get("ariaPressed").as_::<String>()
    }

    pub fn set_aria_pressed(&mut self, value: &str) {
        self.inner.set("ariaPressed", value);
    }
}
impl ElementInternals {
    pub fn aria_read_only(&self) -> String {
        self.inner.get("ariaReadOnly").as_::<String>()
    }

    pub fn set_aria_read_only(&mut self, value: &str) {
        self.inner.set("ariaReadOnly", value);
    }
}
impl ElementInternals {
    pub fn aria_relevant(&self) -> String {
        self.inner.get("ariaRelevant").as_::<String>()
    }

    pub fn set_aria_relevant(&mut self, value: &str) {
        self.inner.set("ariaRelevant", value);
    }
}
impl ElementInternals {
    pub fn aria_required(&self) -> String {
        self.inner.get("ariaRequired").as_::<String>()
    }

    pub fn set_aria_required(&mut self, value: &str) {
        self.inner.set("ariaRequired", value);
    }
}
impl ElementInternals {
    pub fn aria_role_description(&self) -> String {
        self.inner.get("ariaRoleDescription").as_::<String>()
    }

    pub fn set_aria_role_description(&mut self, value: &str) {
        self.inner.set("ariaRoleDescription", value);
    }
}
impl ElementInternals {
    pub fn aria_row_count(&self) -> String {
        self.inner.get("ariaRowCount").as_::<String>()
    }

    pub fn set_aria_row_count(&mut self, value: &str) {
        self.inner.set("ariaRowCount", value);
    }
}
impl ElementInternals {
    pub fn aria_row_index(&self) -> String {
        self.inner.get("ariaRowIndex").as_::<String>()
    }

    pub fn set_aria_row_index(&mut self, value: &str) {
        self.inner.set("ariaRowIndex", value);
    }
}
impl ElementInternals {
    pub fn aria_row_index_text(&self) -> String {
        self.inner.get("ariaRowIndexText").as_::<String>()
    }

    pub fn set_aria_row_index_text(&mut self, value: &str) {
        self.inner.set("ariaRowIndexText", value);
    }
}
impl ElementInternals {
    pub fn aria_row_span(&self) -> String {
        self.inner.get("ariaRowSpan").as_::<String>()
    }

    pub fn set_aria_row_span(&mut self, value: &str) {
        self.inner.set("ariaRowSpan", value);
    }
}
impl ElementInternals {
    pub fn aria_selected(&self) -> String {
        self.inner.get("ariaSelected").as_::<String>()
    }

    pub fn set_aria_selected(&mut self, value: &str) {
        self.inner.set("ariaSelected", value);
    }
}
impl ElementInternals {
    pub fn aria_set_size(&self) -> String {
        self.inner.get("ariaSetSize").as_::<String>()
    }

    pub fn set_aria_set_size(&mut self, value: &str) {
        self.inner.set("ariaSetSize", value);
    }
}
impl ElementInternals {
    pub fn aria_sort(&self) -> String {
        self.inner.get("ariaSort").as_::<String>()
    }

    pub fn set_aria_sort(&mut self, value: &str) {
        self.inner.set("ariaSort", value);
    }
}
impl ElementInternals {
    pub fn aria_value_max(&self) -> String {
        self.inner.get("ariaValueMax").as_::<String>()
    }

    pub fn set_aria_value_max(&mut self, value: &str) {
        self.inner.set("ariaValueMax", value);
    }
}
impl ElementInternals {
    pub fn aria_value_min(&self) -> String {
        self.inner.get("ariaValueMin").as_::<String>()
    }

    pub fn set_aria_value_min(&mut self, value: &str) {
        self.inner.set("ariaValueMin", value);
    }
}
impl ElementInternals {
    pub fn aria_value_now(&self) -> String {
        self.inner.get("ariaValueNow").as_::<String>()
    }

    pub fn set_aria_value_now(&mut self, value: &str) {
        self.inner.set("ariaValueNow", value);
    }
}
impl ElementInternals {
    pub fn aria_value_text(&self) -> String {
        self.inner.get("ariaValueText").as_::<String>()
    }

    pub fn set_aria_value_text(&mut self, value: &str) {
        self.inner.set("ariaValueText", value);
    }
}
