use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PAHistogramContribution {
    inner: emlite::Val,
}
impl FromVal for PAHistogramContribution {
    fn from_val(v: &emlite::Val) -> Self {
        PAHistogramContribution { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PAHistogramContribution {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PAHistogramContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PAHistogramContribution {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PAHistogramContribution {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PAHistogramContribution> for emlite::Val {
    fn from(s: PAHistogramContribution) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    inner: emlite::Val,
}
impl FromVal for PADebugModeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PADebugModeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PADebugModeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PADebugModeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PADebugModeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PADebugModeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PADebugModeOptions> for emlite::Val {
    fn from(s: PADebugModeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PrivateAggregation {
    inner: emlite::Val,
}
impl FromVal for PrivateAggregation {
    fn from_val(v: &emlite::Val) -> Self {
        PrivateAggregation { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PrivateAggregation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PrivateAggregation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PrivateAggregation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PrivateAggregation {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PrivateAggregation> for emlite::Val {
    fn from(s: PrivateAggregation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PrivateAggregation);


impl PrivateAggregation {
    pub fn contribute_to_histogram(&self, contribution: PAHistogramContribution) -> Undefined {
        self.inner.call("contributeToHistogram", &[contribution.into(), ]).as_::<Undefined>()
    }

}
impl PrivateAggregation {
    pub fn contribute_to_histogram_on_event(&self, event: DOMString, contribution: Record<DOMString, Any>) -> Undefined {
        self.inner.call("contributeToHistogramOnEvent", &[event.into(), contribution.into(), ]).as_::<Undefined>()
    }

}
impl PrivateAggregation {
    pub fn enable_debug_mode0(&self, ) -> Undefined {
        self.inner.call("enableDebugMode", &[]).as_::<Undefined>()
    }

    pub fn enable_debug_mode1(&self, options: PADebugModeOptions) -> Undefined {
        self.inner.call("enableDebugMode", &[options.into(), ]).as_::<Undefined>()
    }

}
