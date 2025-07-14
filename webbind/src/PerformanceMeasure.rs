use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceMeasure {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceMeasure {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceMeasure {
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
impl core::ops::Deref for PerformanceMeasure {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMeasure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceMeasure> for emlite::Val {
    fn from(s: PerformanceMeasure) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceMeasure {
    pub fn detail(&self) -> jsbind::Any {
        self.inner.get("detail").as_::<jsbind::Any>()
    }
}
