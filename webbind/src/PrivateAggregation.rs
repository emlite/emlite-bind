use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PAHistogramContribution {
    inner: Any,
}
impl FromVal for PAHistogramContribution {
    fn from_val(v: &Any) -> Self {
        PAHistogramContribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PAHistogramContribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PAHistogramContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PAHistogramContribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PAHistogramContribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PAHistogramContribution> for Any {
    fn from(s: PAHistogramContribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PAHistogramContribution> for Any {
    fn from(s: &PAHistogramContribution) -> Any {
        s.inner.clone()
    }
}

impl PAHistogramContribution {
    pub fn bucket(&self) -> i64 {
        self.inner.get("bucket").as_::<i64>()
    }

    pub fn set_bucket(&mut self, value: i64) {
        self.inner.set("bucket", value);
    }
}
impl PAHistogramContribution {
    pub fn value(&self) -> i32 {
        self.inner.get("value").as_::<i32>()
    }

    pub fn set_value(&mut self, value: i32) {
        self.inner.set("value", value);
    }
}
impl PAHistogramContribution {
    pub fn filtering_id(&self) -> i64 {
        self.inner.get("filteringId").as_::<i64>()
    }

    pub fn set_filtering_id(&mut self, value: i64) {
        self.inner.set("filteringId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PADebugModeOptions {
    inner: Any,
}
impl FromVal for PADebugModeOptions {
    fn from_val(v: &Any) -> Self {
        PADebugModeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PADebugModeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PADebugModeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PADebugModeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PADebugModeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PADebugModeOptions> for Any {
    fn from(s: PADebugModeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PADebugModeOptions> for Any {
    fn from(s: &PADebugModeOptions) -> Any {
        s.inner.clone()
    }
}

impl PADebugModeOptions {
    pub fn debug_key(&self) -> i64 {
        self.inner.get("debugKey").as_::<i64>()
    }

    pub fn set_debug_key(&mut self, value: i64) {
        self.inner.set("debugKey", value);
    }
}
/// The PrivateAggregation class.
/// [`PrivateAggregation`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PrivateAggregation {
    inner: Any,
}
impl FromVal for PrivateAggregation {
    fn from_val(v: &Any) -> Self {
        PrivateAggregation {
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
impl core::ops::Deref for PrivateAggregation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PrivateAggregation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PrivateAggregation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PrivateAggregation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PrivateAggregation> for Any {
    fn from(s: PrivateAggregation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PrivateAggregation> for Any {
    fn from(s: &PrivateAggregation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PrivateAggregation);

impl PrivateAggregation {
    /// The contributeToHistogram method.
    /// [`PrivateAggregation.contributeToHistogram`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation/contributeToHistogram)
    pub fn contribute_to_histogram(&self, contribution: &PAHistogramContribution) -> Undefined {
        self.inner
            .call("contributeToHistogram", &[contribution.into()])
            .as_::<Undefined>()
    }
}
impl PrivateAggregation {
    /// The contributeToHistogramOnEvent method.
    /// [`PrivateAggregation.contributeToHistogramOnEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation/contributeToHistogramOnEvent)
    pub fn contribute_to_histogram_on_event(
        &self,
        event: &str,
        contribution: &Record<String, Any>,
    ) -> Undefined {
        self.inner
            .call(
                "contributeToHistogramOnEvent",
                &[event.into(), contribution.into()],
            )
            .as_::<Undefined>()
    }
}
impl PrivateAggregation {
    /// The enableDebugMode method.
    /// [`PrivateAggregation.enableDebugMode`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation/enableDebugMode)
    pub fn enable_debug_mode0(&self) -> Undefined {
        self.inner.call("enableDebugMode", &[]).as_::<Undefined>()
    }
    /// The enableDebugMode method.
    /// [`PrivateAggregation.enableDebugMode`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation/enableDebugMode)
    pub fn enable_debug_mode1(&self, options: &PADebugModeOptions) -> Undefined {
        self.inner
            .call("enableDebugMode", &[options.into()])
            .as_::<Undefined>()
    }
}
