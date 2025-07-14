use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceLongAnimationFrameTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceLongAnimationFrameTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceLongAnimationFrameTiming {
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
impl core::ops::Deref for PerformanceLongAnimationFrameTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceLongAnimationFrameTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceLongAnimationFrameTiming> for emlite::Val {
    fn from(s: PerformanceLongAnimationFrameTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceLongAnimationFrameTiming {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn entry_type(&self) -> jsbind::DOMString {
        self.inner.get("entryType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn render_start(&self) -> jsbind::Any {
        self.inner.get("renderStart").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn style_and_layout_start(&self) -> jsbind::Any {
        self.inner.get("styleAndLayoutStart").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn blocking_duration(&self) -> jsbind::Any {
        self.inner.get("blockingDuration").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn first_ui_event_timestamp(&self) -> jsbind::Any {
        self.inner.get("firstUIEventTimestamp").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn scripts(&self) -> jsbind::FrozenArray<PerformanceScriptTiming> {
        self.inner
            .get("scripts")
            .as_::<jsbind::FrozenArray<PerformanceScriptTiming>>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn paint_time(&self) -> jsbind::Any {
        self.inner.get("paintTime").as_::<jsbind::Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn presentation_time(&self) -> jsbind::Any {
        self.inner.get("presentationTime").as_::<jsbind::Any>()
    }
}
