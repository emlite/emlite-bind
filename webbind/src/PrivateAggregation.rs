use super::*;

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
        event: &JsString,
        contribution: &Record<JsString, Any>,
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
    pub fn enable_debug_mode(&self) -> Undefined {
        self.inner.call("enableDebugMode", &[]).as_::<Undefined>()
    }
}
impl PrivateAggregation {
    /// The enableDebugMode method.
    /// [`PrivateAggregation.enableDebugMode`](https://developer.mozilla.org/en-US/docs/Web/API/PrivateAggregation/enableDebugMode)
    pub fn enable_debug_mode_with_options(&self, options: &PADebugModeOptions) -> Undefined {
        self.inner
            .call("enableDebugMode", &[options.into()])
            .as_::<Undefined>()
    }
}
