use super::*;

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
