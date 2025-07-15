use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LayoutShift {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutShift {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LayoutShift {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LayoutShift {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LayoutShift> for emlite::Val {
    fn from(s: LayoutShift) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(LayoutShift);

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
    pub fn last_input_time(&self) -> Any {
        self.inner.get("lastInputTime").as_::<Any>()
    }
}
impl LayoutShift {
    pub fn sources(&self) -> FrozenArray<LayoutShiftAttribution> {
        self.inner
            .get("sources")
            .as_::<FrozenArray<LayoutShiftAttribution>>()
    }
}
impl LayoutShift {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
