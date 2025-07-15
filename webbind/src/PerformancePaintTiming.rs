use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformancePaintTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformancePaintTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformancePaintTiming { inner: PerformanceEntry::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformancePaintTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformancePaintTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformancePaintTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformancePaintTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PerformancePaintTiming> for emlite::Val {
    fn from(s: PerformancePaintTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformancePaintTiming);


impl PerformancePaintTiming {
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }

}
impl PerformancePaintTiming {
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }

}
impl PerformancePaintTiming {
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }

}
