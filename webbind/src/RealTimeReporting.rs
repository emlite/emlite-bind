use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RealTimeContribution {
    inner: emlite::Val,
}
impl FromVal for RealTimeContribution {
    fn from_val(v: &emlite::Val) -> Self {
        RealTimeContribution { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RealTimeContribution {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RealTimeContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RealTimeContribution {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RealTimeContribution {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RealTimeContribution> for emlite::Val {
    fn from(s: RealTimeContribution) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RealTimeContribution> for emlite::Val {
    fn from(s: &RealTimeContribution) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RealTimeReporting {
    inner: emlite::Val,
}
impl FromVal for RealTimeReporting {
    fn from_val(v: &emlite::Val) -> Self {
        RealTimeReporting {
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
impl core::ops::Deref for RealTimeReporting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RealTimeReporting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RealTimeReporting {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RealTimeReporting {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RealTimeReporting> for emlite::Val {
    fn from(s: RealTimeReporting) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RealTimeReporting> for emlite::Val {
    fn from(s: &RealTimeReporting) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RealTimeReporting);

impl RealTimeReporting {
    pub fn contribute_to_histogram(&self, contribution: &RealTimeContribution) -> Undefined {
        self.inner
            .call("contributeToHistogram", &[contribution.into()])
            .as_::<Undefined>()
    }
}
