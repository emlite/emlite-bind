use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceEntry {
    inner: emlite::Val,
}
impl FromVal for PerformanceEntry {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceEntry {
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
impl core::ops::Deref for PerformanceEntry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceEntry> for emlite::Val {
    fn from(s: PerformanceEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceEntry);

impl PerformanceEntry {
    pub fn id(&self) -> u64 {
        self.inner.get("id").as_::<u64>()
    }
}
impl PerformanceEntry {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PerformanceEntry {
    pub fn entry_type(&self) -> jsbind::DOMString {
        self.inner.get("entryType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceEntry {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }
}
impl PerformanceEntry {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl PerformanceEntry {
    pub fn navigation_id(&self) -> u64 {
        self.inner.get("navigationId").as_::<u64>()
    }
}
impl PerformanceEntry {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
