use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RealTimeContribution {
    inner: Any,
}
impl FromVal for RealTimeContribution {
    fn from_val(v: &Any) -> Self {
        RealTimeContribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RealTimeContribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RealTimeContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RealTimeContribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RealTimeContribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RealTimeContribution> for Any {
    fn from(s: RealTimeContribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RealTimeContribution> for Any {
    fn from(s: &RealTimeContribution) -> Any {
        s.inner.clone()
    }
}

impl RealTimeContribution {
    pub fn bucket(&self) -> i32 {
        self.inner.get("bucket").as_::<i32>()
    }

    pub fn set_bucket(&mut self, value: i32) {
        self.inner.set("bucket", value);
    }
}
impl RealTimeContribution {
    pub fn priority_weight(&self) -> f64 {
        self.inner.get("priorityWeight").as_::<f64>()
    }

    pub fn set_priority_weight(&mut self, value: f64) {
        self.inner.set("priorityWeight", value);
    }
}
impl RealTimeContribution {
    pub fn latency_threshold(&self) -> i32 {
        self.inner.get("latencyThreshold").as_::<i32>()
    }

    pub fn set_latency_threshold(&mut self, value: i32) {
        self.inner.set("latencyThreshold", value);
    }
}
/// The RealTimeReporting class.
/// [`RealTimeReporting`](https://developer.mozilla.org/en-US/docs/Web/API/RealTimeReporting)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RealTimeReporting {
    inner: Any,
}
impl FromVal for RealTimeReporting {
    fn from_val(v: &Any) -> Self {
        RealTimeReporting {
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
impl core::ops::Deref for RealTimeReporting {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RealTimeReporting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RealTimeReporting {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RealTimeReporting {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RealTimeReporting> for Any {
    fn from(s: RealTimeReporting) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RealTimeReporting> for Any {
    fn from(s: &RealTimeReporting) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RealTimeReporting);

impl RealTimeReporting {
    /// The contributeToHistogram method.
    /// [`RealTimeReporting.contributeToHistogram`](https://developer.mozilla.org/en-US/docs/Web/API/RealTimeReporting/contributeToHistogram)
    pub fn contribute_to_histogram(&self, contribution: &RealTimeContribution) -> Undefined {
        self.inner
            .call("contributeToHistogram", &[contribution.into()])
            .as_::<Undefined>()
    }
}
