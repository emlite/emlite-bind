use super::*;

#[derive(Clone, Debug)]
pub struct LayoutShift {
    inner: PerformanceEntry,
}
impl FromVal for LayoutShift {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutShift {
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
impl std::ops::Deref for LayoutShift {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LayoutShift {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LayoutShift> for emlite::Val {
    fn from(s: LayoutShift) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutShift {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
impl LayoutShift {
    pub fn had_recent_input(&self) -> bool {
        self.inner.get("hadRecentInput").as_::<bool>()
    }
}
impl LayoutShift {
    pub fn last_input_time(&self) -> jsbind::Any {
        self.inner.get("lastInputTime").as_::<jsbind::Any>()
    }
}
impl LayoutShift {
    pub fn sources(&self) -> jsbind::FrozenArray<LayoutShiftAttribution> {
        self.inner
            .get("sources")
            .as_::<jsbind::FrozenArray<LayoutShiftAttribution>>()
    }
}
impl LayoutShift {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
