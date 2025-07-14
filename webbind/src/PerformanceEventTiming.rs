use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceEventTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceEventTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceEventTiming {
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
impl core::ops::Deref for PerformanceEventTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceEventTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceEventTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceEventTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceEventTiming> for emlite::Val {
    fn from(s: PerformanceEventTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceEventTiming);

impl PerformanceEventTiming {
    pub fn processing_start(&self) -> jsbind::Any {
        self.inner.get("processingStart").as_::<jsbind::Any>()
    }
}
impl PerformanceEventTiming {
    pub fn processing_end(&self) -> jsbind::Any {
        self.inner.get("processingEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceEventTiming {
    pub fn cancelable(&self) -> bool {
        self.inner.get("cancelable").as_::<bool>()
    }
}
impl PerformanceEventTiming {
    pub fn target(&self) -> Node {
        self.inner.get("target").as_::<Node>()
    }
}
impl PerformanceEventTiming {
    pub fn interaction_id(&self) -> u64 {
        self.inner.get("interactionId").as_::<u64>()
    }
}
impl PerformanceEventTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
