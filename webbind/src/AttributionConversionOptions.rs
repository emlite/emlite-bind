use super::*;




/// The AttributionConversionOptions dictionary.
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
    /// Getter of the `aggregationService` attribute.
    pub fn aggregation_service(&self) -> JsString {
        self.inner.get("aggregationService").as_::<JsString>()
    }

    /// Setter of the `aggregationService` attribute.
    pub fn set_aggregation_service(&mut self, value: &JsString) {
        self.inner.set("aggregationService", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `epsilon` attribute.
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    /// Setter of the `epsilon` attribute.
    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `histogramSize` attribute.
    pub fn histogram_size(&self) -> u32 {
        self.inner.get("histogramSize").as_::<u32>()
    }

    /// Setter of the `histogramSize` attribute.
    pub fn set_histogram_size(&mut self, value: u32) {
        self.inner.set("histogramSize", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `lookbackDays` attribute.
    pub fn lookback_days(&self) -> u32 {
        self.inner.get("lookbackDays").as_::<u32>()
    }

    /// Setter of the `lookbackDays` attribute.
    pub fn set_lookback_days(&mut self, value: u32) {
        self.inner.set("lookbackDays", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `matchValues` attribute.
    pub fn match_values(&self) -> TypedArray<u32> {
        self.inner.get("matchValues").as_::<TypedArray<u32>>()
    }

    /// Setter of the `matchValues` attribute.
    pub fn set_match_values(&mut self, value: TypedArray<u32>) {
        self.inner.set("matchValues", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `impressionSites` attribute.
    pub fn impression_sites(&self) -> TypedArray<JsString> {
        self.inner.get("impressionSites").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `impressionSites` attribute.
    pub fn set_impression_sites(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("impressionSites", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `impressionCallers` attribute.
    pub fn impression_callers(&self) -> TypedArray<JsString> {
        self.inner.get("impressionCallers").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `impressionCallers` attribute.
    pub fn set_impression_callers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("impressionCallers", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `logic` attribute.
    pub fn logic(&self) -> AttributionLogic {
        self.inner.get("logic").as_::<AttributionLogic>()
    }

    /// Setter of the `logic` attribute.
    pub fn set_logic(&mut self, value: &AttributionLogic) {
        self.inner.set("logic", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `logicOptions` attribute.
    pub fn logic_options(&self) -> AttributionLogicOptions {
        self.inner.get("logicOptions").as_::<AttributionLogicOptions>()
    }

    /// Setter of the `logicOptions` attribute.
    pub fn set_logic_options(&mut self, value: &AttributionLogicOptions) {
        self.inner.set("logicOptions", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> u32 {
        self.inner.get("value").as_::<u32>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: u32) {
        self.inner.set("value", value);
    }
}
impl AttributionConversionOptions {
    /// Getter of the `maxValue` attribute.
    pub fn max_value(&self) -> u32 {
        self.inner.get("maxValue").as_::<u32>()
    }

    /// Setter of the `maxValue` attribute.
    pub fn set_max_value(&mut self, value: u32) {
        self.inner.set("maxValue", value);
    }
}
