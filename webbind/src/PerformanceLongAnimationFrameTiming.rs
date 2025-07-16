use super::*;

/// The PerformanceLongAnimationFrameTiming class.
/// [`PerformanceLongAnimationFrameTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceLongAnimationFrameTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceLongAnimationFrameTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceLongAnimationFrameTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for PerformanceLongAnimationFrameTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceLongAnimationFrameTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceLongAnimationFrameTiming> for Any {
    fn from(s: PerformanceLongAnimationFrameTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceLongAnimationFrameTiming> for Any {
    fn from(s: &PerformanceLongAnimationFrameTiming) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceLongAnimationFrameTiming);

impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `startTime` attribute.
    /// [`PerformanceLongAnimationFrameTiming.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `duration` attribute.
    /// [`PerformanceLongAnimationFrameTiming.duration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `name` attribute.
    /// [`PerformanceLongAnimationFrameTiming.name`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `entryType` attribute.
    /// [`PerformanceLongAnimationFrameTiming.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/entryType)
    pub fn entry_type(&self) -> String {
        self.inner.get("entryType").as_::<String>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `renderStart` attribute.
    /// [`PerformanceLongAnimationFrameTiming.renderStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/renderStart)
    pub fn render_start(&self) -> Any {
        self.inner.get("renderStart").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `styleAndLayoutStart` attribute.
    /// [`PerformanceLongAnimationFrameTiming.styleAndLayoutStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/styleAndLayoutStart)
    pub fn style_and_layout_start(&self) -> Any {
        self.inner.get("styleAndLayoutStart").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `blockingDuration` attribute.
    /// [`PerformanceLongAnimationFrameTiming.blockingDuration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/blockingDuration)
    pub fn blocking_duration(&self) -> Any {
        self.inner.get("blockingDuration").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `firstUIEventTimestamp` attribute.
    /// [`PerformanceLongAnimationFrameTiming.firstUIEventTimestamp`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/firstUIEventTimestamp)
    pub fn first_ui_event_timestamp(&self) -> Any {
        self.inner.get("firstUIEventTimestamp").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `scripts` attribute.
    /// [`PerformanceLongAnimationFrameTiming.scripts`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/scripts)
    pub fn scripts(&self) -> FrozenArray<PerformanceScriptTiming> {
        self.inner
            .get("scripts")
            .as_::<FrozenArray<PerformanceScriptTiming>>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// The toJSON method.
    /// [`PerformanceLongAnimationFrameTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `paintTime` attribute.
    /// [`PerformanceLongAnimationFrameTiming.paintTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/paintTime)
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }
}
impl PerformanceLongAnimationFrameTiming {
    /// Getter of the `presentationTime` attribute.
    /// [`PerformanceLongAnimationFrameTiming.presentationTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongAnimationFrameTiming/presentationTime)
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }
}
