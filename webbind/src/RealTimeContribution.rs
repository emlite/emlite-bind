use super::*;

/// The RealTimeContribution dictionary.
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
    /// Getter of the `bucket` attribute.
    pub fn bucket(&self) -> i32 {
        self.inner.get("bucket").as_::<i32>()
    }

    /// Setter of the `bucket` attribute.
    pub fn set_bucket(&mut self, value: i32) {
        self.inner.set("bucket", value);
    }
}
impl RealTimeContribution {
    /// Getter of the `priorityWeight` attribute.
    pub fn priority_weight(&self) -> f64 {
        self.inner.get("priorityWeight").as_::<f64>()
    }

    /// Setter of the `priorityWeight` attribute.
    pub fn set_priority_weight(&mut self, value: f64) {
        self.inner.set("priorityWeight", value);
    }
}
impl RealTimeContribution {
    /// Getter of the `latencyThreshold` attribute.
    pub fn latency_threshold(&self) -> i32 {
        self.inner.get("latencyThreshold").as_::<i32>()
    }

    /// Setter of the `latencyThreshold` attribute.
    pub fn set_latency_threshold(&mut self, value: i32) {
        self.inner.set("latencyThreshold", value);
    }
}
