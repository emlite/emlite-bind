use super::*;




/// The HIDReportItem dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDReportItem {
    inner: Any,
}

impl FromVal for HIDReportItem {
    fn from_val(v: &Any) -> Self {
        HIDReportItem { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDReportItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDReportItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDReportItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDReportItem {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HIDReportItem> for Any {
    fn from(s: HIDReportItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDReportItem> for Any {
    fn from(s: &HIDReportItem) -> Any {
        s.inner.clone()
    }
}

impl HIDReportItem {
    /// Getter of the `isAbsolute` attribute.
    pub fn is_absolute(&self) -> bool {
        self.inner.get("isAbsolute").as_::<bool>()
    }

    /// Setter of the `isAbsolute` attribute.
    pub fn set_is_absolute(&mut self, value: bool) {
        self.inner.set("isAbsolute", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isArray` attribute.
    pub fn is_array(&self) -> bool {
        self.inner.get("isArray").as_::<bool>()
    }

    /// Setter of the `isArray` attribute.
    pub fn set_is_array(&mut self, value: bool) {
        self.inner.set("isArray", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isBufferedBytes` attribute.
    pub fn is_buffered_bytes(&self) -> bool {
        self.inner.get("isBufferedBytes").as_::<bool>()
    }

    /// Setter of the `isBufferedBytes` attribute.
    pub fn set_is_buffered_bytes(&mut self, value: bool) {
        self.inner.set("isBufferedBytes", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isConstant` attribute.
    pub fn is_constant(&self) -> bool {
        self.inner.get("isConstant").as_::<bool>()
    }

    /// Setter of the `isConstant` attribute.
    pub fn set_is_constant(&mut self, value: bool) {
        self.inner.set("isConstant", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isLinear` attribute.
    pub fn is_linear(&self) -> bool {
        self.inner.get("isLinear").as_::<bool>()
    }

    /// Setter of the `isLinear` attribute.
    pub fn set_is_linear(&mut self, value: bool) {
        self.inner.set("isLinear", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isRange` attribute.
    pub fn is_range(&self) -> bool {
        self.inner.get("isRange").as_::<bool>()
    }

    /// Setter of the `isRange` attribute.
    pub fn set_is_range(&mut self, value: bool) {
        self.inner.set("isRange", value);
    }
}
impl HIDReportItem {
    /// Getter of the `isVolatile` attribute.
    pub fn is_volatile(&self) -> bool {
        self.inner.get("isVolatile").as_::<bool>()
    }

    /// Setter of the `isVolatile` attribute.
    pub fn set_is_volatile(&mut self, value: bool) {
        self.inner.set("isVolatile", value);
    }
}
impl HIDReportItem {
    /// Getter of the `hasNull` attribute.
    pub fn has_null(&self) -> bool {
        self.inner.get("hasNull").as_::<bool>()
    }

    /// Setter of the `hasNull` attribute.
    pub fn set_has_null(&mut self, value: bool) {
        self.inner.set("hasNull", value);
    }
}
impl HIDReportItem {
    /// Getter of the `hasPreferredState` attribute.
    pub fn has_preferred_state(&self) -> bool {
        self.inner.get("hasPreferredState").as_::<bool>()
    }

    /// Setter of the `hasPreferredState` attribute.
    pub fn set_has_preferred_state(&mut self, value: bool) {
        self.inner.set("hasPreferredState", value);
    }
}
impl HIDReportItem {
    /// Getter of the `wrap` attribute.
    pub fn wrap(&self) -> bool {
        self.inner.get("wrap").as_::<bool>()
    }

    /// Setter of the `wrap` attribute.
    pub fn set_wrap(&mut self, value: bool) {
        self.inner.set("wrap", value);
    }
}
impl HIDReportItem {
    /// Getter of the `usages` attribute.
    pub fn usages(&self) -> TypedArray<u32> {
        self.inner.get("usages").as_::<TypedArray<u32>>()
    }

    /// Setter of the `usages` attribute.
    pub fn set_usages(&mut self, value: TypedArray<u32>) {
        self.inner.set("usages", value);
    }
}
impl HIDReportItem {
    /// Getter of the `usageMinimum` attribute.
    pub fn usage_minimum(&self) -> u32 {
        self.inner.get("usageMinimum").as_::<u32>()
    }

    /// Setter of the `usageMinimum` attribute.
    pub fn set_usage_minimum(&mut self, value: u32) {
        self.inner.set("usageMinimum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `usageMaximum` attribute.
    pub fn usage_maximum(&self) -> u32 {
        self.inner.get("usageMaximum").as_::<u32>()
    }

    /// Setter of the `usageMaximum` attribute.
    pub fn set_usage_maximum(&mut self, value: u32) {
        self.inner.set("usageMaximum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `reportSize` attribute.
    pub fn report_size(&self) -> u16 {
        self.inner.get("reportSize").as_::<u16>()
    }

    /// Setter of the `reportSize` attribute.
    pub fn set_report_size(&mut self, value: u16) {
        self.inner.set("reportSize", value);
    }
}
impl HIDReportItem {
    /// Getter of the `reportCount` attribute.
    pub fn report_count(&self) -> u16 {
        self.inner.get("reportCount").as_::<u16>()
    }

    /// Setter of the `reportCount` attribute.
    pub fn set_report_count(&mut self, value: u16) {
        self.inner.set("reportCount", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitExponent` attribute.
    pub fn unit_exponent(&self) -> i8 {
        self.inner.get("unitExponent").as_::<i8>()
    }

    /// Setter of the `unitExponent` attribute.
    pub fn set_unit_exponent(&mut self, value: i8) {
        self.inner.set("unitExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitSystem` attribute.
    pub fn unit_system(&self) -> HIDUnitSystem {
        self.inner.get("unitSystem").as_::<HIDUnitSystem>()
    }

    /// Setter of the `unitSystem` attribute.
    pub fn set_unit_system(&mut self, value: &HIDUnitSystem) {
        self.inner.set("unitSystem", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorLengthExponent` attribute.
    pub fn unit_factor_length_exponent(&self) -> i8 {
        self.inner.get("unitFactorLengthExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorLengthExponent` attribute.
    pub fn set_unit_factor_length_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorLengthExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorMassExponent` attribute.
    pub fn unit_factor_mass_exponent(&self) -> i8 {
        self.inner.get("unitFactorMassExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorMassExponent` attribute.
    pub fn set_unit_factor_mass_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorMassExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorTimeExponent` attribute.
    pub fn unit_factor_time_exponent(&self) -> i8 {
        self.inner.get("unitFactorTimeExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorTimeExponent` attribute.
    pub fn set_unit_factor_time_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorTimeExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorTemperatureExponent` attribute.
    pub fn unit_factor_temperature_exponent(&self) -> i8 {
        self.inner.get("unitFactorTemperatureExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorTemperatureExponent` attribute.
    pub fn set_unit_factor_temperature_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorTemperatureExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorCurrentExponent` attribute.
    pub fn unit_factor_current_exponent(&self) -> i8 {
        self.inner.get("unitFactorCurrentExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorCurrentExponent` attribute.
    pub fn set_unit_factor_current_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorCurrentExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `unitFactorLuminousIntensityExponent` attribute.
    pub fn unit_factor_luminous_intensity_exponent(&self) -> i8 {
        self.inner.get("unitFactorLuminousIntensityExponent").as_::<i8>()
    }

    /// Setter of the `unitFactorLuminousIntensityExponent` attribute.
    pub fn set_unit_factor_luminous_intensity_exponent(&mut self, value: i8) {
        self.inner.set("unitFactorLuminousIntensityExponent", value);
    }
}
impl HIDReportItem {
    /// Getter of the `logicalMinimum` attribute.
    pub fn logical_minimum(&self) -> i32 {
        self.inner.get("logicalMinimum").as_::<i32>()
    }

    /// Setter of the `logicalMinimum` attribute.
    pub fn set_logical_minimum(&mut self, value: i32) {
        self.inner.set("logicalMinimum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `logicalMaximum` attribute.
    pub fn logical_maximum(&self) -> i32 {
        self.inner.get("logicalMaximum").as_::<i32>()
    }

    /// Setter of the `logicalMaximum` attribute.
    pub fn set_logical_maximum(&mut self, value: i32) {
        self.inner.set("logicalMaximum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `physicalMinimum` attribute.
    pub fn physical_minimum(&self) -> i32 {
        self.inner.get("physicalMinimum").as_::<i32>()
    }

    /// Setter of the `physicalMinimum` attribute.
    pub fn set_physical_minimum(&mut self, value: i32) {
        self.inner.set("physicalMinimum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `physicalMaximum` attribute.
    pub fn physical_maximum(&self) -> i32 {
        self.inner.get("physicalMaximum").as_::<i32>()
    }

    /// Setter of the `physicalMaximum` attribute.
    pub fn set_physical_maximum(&mut self, value: i32) {
        self.inner.set("physicalMaximum", value);
    }
}
impl HIDReportItem {
    /// Getter of the `strings` attribute.
    pub fn strings(&self) -> TypedArray<JsString> {
        self.inner.get("strings").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `strings` attribute.
    pub fn set_strings(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("strings", value);
    }
}
