use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PerformanceLongAnimationFrameTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceLongAnimationFrameTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&PerformanceLongAnimationFrameTiming> for emlite::Val {
    fn from(s: &PerformanceLongAnimationFrameTiming) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceLongAnimationFrameTiming);

impl PerformanceLongAnimationFrameTiming {
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn entry_type(&self) -> DOMString {
        self.inner.get("entryType").as_::<DOMString>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn render_start(&self) -> Any {
        self.inner.get("renderStart").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn style_and_layout_start(&self) -> Any {
        self.inner.get("styleAndLayoutStart").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn blocking_duration(&self) -> Any {
        self.inner.get("blockingDuration").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn first_ui_event_timestamp(&self) -> Any {
        self.inner.get("firstUIEventTimestamp").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn scripts(&self) -> FrozenArray<PerformanceScriptTiming> {
        self.inner
            .get("scripts")
            .as_::<FrozenArray<PerformanceScriptTiming>>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }
}
