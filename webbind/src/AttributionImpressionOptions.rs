use super::*;




/// The AttributionImpressionOptions dictionary.
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
    /// Getter of the `histogramIndex` attribute.
    pub fn histogram_index(&self) -> u32 {
        self.inner.get("histogramIndex").as_::<u32>()
    }

    /// Setter of the `histogramIndex` attribute.
    pub fn set_histogram_index(&mut self, value: u32) {
        self.inner.set("histogramIndex", value);
    }
}
impl AttributionImpressionOptions {
    /// Getter of the `matchValue` attribute.
    pub fn match_value(&self) -> u32 {
        self.inner.get("matchValue").as_::<u32>()
    }

    /// Setter of the `matchValue` attribute.
    pub fn set_match_value(&mut self, value: u32) {
        self.inner.set("matchValue", value);
    }
}
impl AttributionImpressionOptions {
    /// Getter of the `conversionSites` attribute.
    pub fn conversion_sites(&self) -> TypedArray<JsString> {
        self.inner.get("conversionSites").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `conversionSites` attribute.
    pub fn set_conversion_sites(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("conversionSites", value);
    }
}
impl AttributionImpressionOptions {
    /// Getter of the `conversionCallers` attribute.
    pub fn conversion_callers(&self) -> TypedArray<JsString> {
        self.inner.get("conversionCallers").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `conversionCallers` attribute.
    pub fn set_conversion_callers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("conversionCallers", value);
    }
}
impl AttributionImpressionOptions {
    /// Getter of the `lifetimeDays` attribute.
    pub fn lifetime_days(&self) -> u32 {
        self.inner.get("lifetimeDays").as_::<u32>()
    }

    /// Setter of the `lifetimeDays` attribute.
    pub fn set_lifetime_days(&mut self, value: u32) {
        self.inner.set("lifetimeDays", value);
    }
}
