use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionImpressionResult {
    inner: Any,
}
impl FromVal for AttributionImpressionResult {
    fn from_val(v: &Any) -> Self {
        AttributionImpressionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionImpressionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionImpressionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionImpressionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionImpressionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionImpressionResult> for Any {
    fn from(s: AttributionImpressionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionImpressionResult> for Any {
    fn from(s: &AttributionImpressionResult) -> Any {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionImpressionOptions {
    inner: Any,
}
impl FromVal for AttributionImpressionOptions {
    fn from_val(v: &Any) -> Self {
        AttributionImpressionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionImpressionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionImpressionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionImpressionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionImpressionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionImpressionOptions> for Any {
    fn from(s: AttributionImpressionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionImpressionOptions> for Any {
    fn from(s: &AttributionImpressionOptions) -> Any {
        s.inner.clone()
    }
}

impl AttributionImpressionOptions {
    pub fn histogram_index(&self) -> u32 {
        self.inner.get("histogramIndex").as_::<u32>()
    }

    pub fn set_histogram_index(&mut self, value: u32) {
        self.inner.set("histogramIndex", value);
    }
}
impl AttributionImpressionOptions {
    pub fn match_value(&self) -> u32 {
        self.inner.get("matchValue").as_::<u32>()
    }

    pub fn set_match_value(&mut self, value: u32) {
        self.inner.set("matchValue", value);
    }
}
impl AttributionImpressionOptions {
    pub fn conversion_sites(&self) -> TypedArray<JsString> {
        self.inner
            .get("conversionSites")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_conversion_sites(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("conversionSites", value);
    }
}
impl AttributionImpressionOptions {
    pub fn conversion_callers(&self) -> TypedArray<JsString> {
        self.inner
            .get("conversionCallers")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_conversion_callers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("conversionCallers", value);
    }
}
impl AttributionImpressionOptions {
    pub fn lifetime_days(&self) -> u32 {
        self.inner.get("lifetimeDays").as_::<u32>()
    }

    pub fn set_lifetime_days(&mut self, value: u32) {
        self.inner.set("lifetimeDays", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionConversionResult {
    inner: Any,
}
impl FromVal for AttributionConversionResult {
    fn from_val(v: &Any) -> Self {
        AttributionConversionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionConversionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionConversionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionConversionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionConversionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionConversionResult> for Any {
    fn from(s: AttributionConversionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionConversionResult> for Any {
    fn from(s: &AttributionConversionResult) -> Any {
        s.inner.clone()
    }
}

impl AttributionConversionResult {
    pub fn report(&self) -> Uint8Array {
        self.inner.get("report").as_::<Uint8Array>()
    }

    pub fn set_report(&mut self, value: &Uint8Array) {
        self.inner.set("report", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionConversionOptions {
    inner: Any,
}
impl FromVal for AttributionConversionOptions {
    fn from_val(v: &Any) -> Self {
        AttributionConversionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionConversionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionConversionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionConversionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionConversionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionConversionOptions> for Any {
    fn from(s: AttributionConversionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionConversionOptions> for Any {
    fn from(s: &AttributionConversionOptions) -> Any {
        s.inner.clone()
    }
}

impl AttributionConversionOptions {
    pub fn aggregation_service(&self) -> JsString {
        self.inner.get("aggregationService").as_::<JsString>()
    }

    pub fn set_aggregation_service(&mut self, value: &JsString) {
        self.inner.set("aggregationService", value);
    }
}
impl AttributionConversionOptions {
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
impl AttributionConversionOptions {
    pub fn histogram_size(&self) -> u32 {
        self.inner.get("histogramSize").as_::<u32>()
    }

    pub fn set_histogram_size(&mut self, value: u32) {
        self.inner.set("histogramSize", value);
    }
}
impl AttributionConversionOptions {
    pub fn lookback_days(&self) -> u32 {
        self.inner.get("lookbackDays").as_::<u32>()
    }

    pub fn set_lookback_days(&mut self, value: u32) {
        self.inner.set("lookbackDays", value);
    }
}
impl AttributionConversionOptions {
    pub fn match_values(&self) -> TypedArray<u32> {
        self.inner.get("matchValues").as_::<TypedArray<u32>>()
    }

    pub fn set_match_values(&mut self, value: TypedArray<u32>) {
        self.inner.set("matchValues", value);
    }
}
impl AttributionConversionOptions {
    pub fn impression_sites(&self) -> TypedArray<JsString> {
        self.inner
            .get("impressionSites")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_impression_sites(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("impressionSites", value);
    }
}
impl AttributionConversionOptions {
    pub fn impression_callers(&self) -> TypedArray<JsString> {
        self.inner
            .get("impressionCallers")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_impression_callers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("impressionCallers", value);
    }
}
impl AttributionConversionOptions {
    pub fn logic(&self) -> AttributionLogic {
        self.inner.get("logic").as_::<AttributionLogic>()
    }

    pub fn set_logic(&mut self, value: &AttributionLogic) {
        self.inner.set("logic", value);
    }
}
impl AttributionConversionOptions {
    pub fn value(&self) -> u32 {
        self.inner.get("value").as_::<u32>()
    }

    pub fn set_value(&mut self, value: u32) {
        self.inner.set("value", value);
    }
}
impl AttributionConversionOptions {
    pub fn max_value(&self) -> u32 {
        self.inner.get("maxValue").as_::<u32>()
    }

    pub fn set_max_value(&mut self, value: u32) {
        self.inner.set("maxValue", value);
    }
}
/// The Attribution class.
/// [`Attribution`](https://developer.mozilla.org/en-US/docs/Web/API/Attribution)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Attribution {
    inner: Any,
}
impl FromVal for Attribution {
    fn from_val(v: &Any) -> Self {
        Attribution {
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
impl core::ops::Deref for Attribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Attribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Attribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Attribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Attribution> for Any {
    fn from(s: Attribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Attribution> for Any {
    fn from(s: &Attribution) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Attribution);

impl Attribution {
    /// Getter of the `aggregationServices` attribute.
    /// [`Attribution.aggregationServices`](https://developer.mozilla.org/en-US/docs/Web/API/Attribution/aggregationServices)
    pub fn aggregation_services(&self) -> AttributionAggregationServices {
        self.inner
            .get("aggregationServices")
            .as_::<AttributionAggregationServices>()
    }
}
impl Attribution {
    /// The saveImpression method.
    /// [`Attribution.saveImpression`](https://developer.mozilla.org/en-US/docs/Web/API/Attribution/saveImpression)
    pub fn save_impression(
        &self,
        options: &AttributionImpressionOptions,
    ) -> Promise<AttributionImpressionResult> {
        self.inner
            .call("saveImpression", &[options.into()])
            .as_::<Promise<AttributionImpressionResult>>()
    }
}
impl Attribution {
    /// The measureConversion method.
    /// [`Attribution.measureConversion`](https://developer.mozilla.org/en-US/docs/Web/API/Attribution/measureConversion)
    pub fn measure_conversion(
        &self,
        options: &AttributionConversionOptions,
    ) -> Promise<AttributionConversionResult> {
        self.inner
            .call("measureConversion", &[options.into()])
            .as_::<Promise<AttributionConversionResult>>()
    }
}
