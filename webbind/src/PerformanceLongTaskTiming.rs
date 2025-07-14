use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceLongTaskTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceLongTaskTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceLongTaskTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceLongTaskTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceLongTaskTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceLongTaskTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceLongTaskTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceLongTaskTiming> for emlite::Val {
    fn from(s: PerformanceLongTaskTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceLongTaskTiming);

impl PerformanceLongTaskTiming {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }
}
impl PerformanceLongTaskTiming {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl PerformanceLongTaskTiming {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PerformanceLongTaskTiming {
    pub fn entry_type(&self) -> jsbind::DOMString {
        self.inner.get("entryType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceLongTaskTiming {
    pub fn attribution(&self) -> jsbind::FrozenArray<TaskAttributionTiming> {
        self.inner
            .get("attribution")
            .as_::<jsbind::FrozenArray<TaskAttributionTiming>>()
    }
}
impl PerformanceLongTaskTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
