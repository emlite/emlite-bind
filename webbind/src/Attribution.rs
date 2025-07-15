use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionImpressionResult {
    inner: emlite::Val,
}
impl FromVal for AttributionImpressionResult {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionImpressionResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionImpressionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionImpressionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AttributionImpressionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AttributionImpressionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AttributionImpressionResult> for emlite::Val {
    fn from(s: AttributionImpressionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AttributionImpressionResult> for emlite::Val {
    fn from(s: &AttributionImpressionResult) -> emlite::Val {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionImpressionOptions {
    inner: emlite::Val,
}
impl FromVal for AttributionImpressionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionImpressionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionImpressionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionImpressionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AttributionImpressionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AttributionImpressionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AttributionImpressionOptions> for emlite::Val {
    fn from(s: AttributionImpressionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AttributionImpressionOptions> for emlite::Val {
    fn from(s: &AttributionImpressionOptions) -> emlite::Val {
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
    pub fn conversion_sites(&self) -> Sequence<USVString> {
        self.inner
            .get("conversionSites")
            .as_::<Sequence<USVString>>()
    }

    pub fn set_conversion_sites(&mut self, value: Sequence<USVString>) {
        self.inner.set("conversionSites", value);
    }
}
impl AttributionImpressionOptions {
    pub fn conversion_callers(&self) -> Sequence<USVString> {
        self.inner
            .get("conversionCallers")
            .as_::<Sequence<USVString>>()
    }

    pub fn set_conversion_callers(&mut self, value: Sequence<USVString>) {
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
    inner: emlite::Val,
}
impl FromVal for AttributionConversionResult {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionConversionResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionConversionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionConversionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AttributionConversionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AttributionConversionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AttributionConversionResult> for emlite::Val {
    fn from(s: AttributionConversionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AttributionConversionResult> for emlite::Val {
    fn from(s: &AttributionConversionResult) -> emlite::Val {
        s.inner.clone()
    }
}

impl AttributionConversionResult {
    pub fn report(&self) -> Uint8Array {
        self.inner.get("report").as_::<Uint8Array>()
    }

    pub fn set_report(&mut self, value: Uint8Array) {
        self.inner.set("report", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionConversionOptions {
    inner: emlite::Val,
}
impl FromVal for AttributionConversionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionConversionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionConversionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionConversionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AttributionConversionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AttributionConversionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AttributionConversionOptions> for emlite::Val {
    fn from(s: AttributionConversionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AttributionConversionOptions> for emlite::Val {
    fn from(s: &AttributionConversionOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl AttributionConversionOptions {
    pub fn aggregation_service(&self) -> USVString {
        self.inner.get("aggregationService").as_::<USVString>()
    }

    pub fn set_aggregation_service(&mut self, value: USVString) {
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
    pub fn match_values(&self) -> Sequence<u32> {
        self.inner.get("matchValues").as_::<Sequence<u32>>()
    }

    pub fn set_match_values(&mut self, value: Sequence<u32>) {
        self.inner.set("matchValues", value);
    }
}
impl AttributionConversionOptions {
    pub fn impression_sites(&self) -> Sequence<USVString> {
        self.inner
            .get("impressionSites")
            .as_::<Sequence<USVString>>()
    }

    pub fn set_impression_sites(&mut self, value: Sequence<USVString>) {
        self.inner.set("impressionSites", value);
    }
}
impl AttributionConversionOptions {
    pub fn impression_callers(&self) -> Sequence<USVString> {
        self.inner
            .get("impressionCallers")
            .as_::<Sequence<USVString>>()
    }

    pub fn set_impression_callers(&mut self, value: Sequence<USVString>) {
        self.inner.set("impressionCallers", value);
    }
}
impl AttributionConversionOptions {
    pub fn logic(&self) -> AttributionLogic {
        self.inner.get("logic").as_::<AttributionLogic>()
    }

    pub fn set_logic(&mut self, value: AttributionLogic) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Attribution {
    inner: emlite::Val,
}
impl FromVal for Attribution {
    fn from_val(v: &emlite::Val) -> Self {
        Attribution {
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
impl core::ops::Deref for Attribution {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Attribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Attribution {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Attribution {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Attribution> for emlite::Val {
    fn from(s: Attribution) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Attribution> for emlite::Val {
    fn from(s: &Attribution) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Attribution);

impl Attribution {
    pub fn aggregation_services(&self) -> AttributionAggregationServices {
        self.inner
            .get("aggregationServices")
            .as_::<AttributionAggregationServices>()
    }
}
impl Attribution {
    pub fn save_impression(&self, options: AttributionImpressionOptions) -> Promise {
        self.inner
            .call("saveImpression", &[options.into()])
            .as_::<Promise>()
    }
}
impl Attribution {
    pub fn measure_conversion(&self, options: AttributionConversionOptions) -> Promise {
        self.inner
            .call("measureConversion", &[options.into()])
            .as_::<Promise>()
    }
}
