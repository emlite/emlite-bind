use super::*;

#[derive(Clone, Debug)]
pub struct PerformancePaintTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformancePaintTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformancePaintTiming {
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
impl std::ops::Deref for PerformancePaintTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformancePaintTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformancePaintTiming> for emlite::Val {
    fn from(s: PerformancePaintTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformancePaintTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl PerformancePaintTiming {
    pub fn paint_time(&self) -> jsbind::Any {
        self.inner.get("paintTime").as_::<jsbind::Any>()
    }
}
impl PerformancePaintTiming {
    pub fn presentation_time(&self) -> jsbind::Any {
        self.inner.get("presentationTime").as_::<jsbind::Any>()
    }
}
