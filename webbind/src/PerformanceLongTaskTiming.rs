use super::*;

/// The PerformanceLongTaskTiming class.
/// [`PerformanceLongTaskTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceLongTaskTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceLongTaskTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceLongTaskTiming {
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
impl AsRef<Any> for PerformanceLongTaskTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceLongTaskTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceLongTaskTiming> for Any {
    fn from(s: PerformanceLongTaskTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceLongTaskTiming> for Any {
    fn from(s: &PerformanceLongTaskTiming) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceLongTaskTiming);

impl PerformanceLongTaskTiming {
    /// Getter of the `startTime` attribute.
    /// [`PerformanceLongTaskTiming.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl PerformanceLongTaskTiming {
    /// Getter of the `duration` attribute.
    /// [`PerformanceLongTaskTiming.duration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceLongTaskTiming {
    /// Getter of the `name` attribute.
    /// [`PerformanceLongTaskTiming.name`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl PerformanceLongTaskTiming {
    /// Getter of the `entryType` attribute.
    /// [`PerformanceLongTaskTiming.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/entryType)
    pub fn entry_type(&self) -> DOMString {
        self.inner.get("entryType").as_::<DOMString>()
    }
}
impl PerformanceLongTaskTiming {
    /// Getter of the `attribution` attribute.
    /// [`PerformanceLongTaskTiming.attribution`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/attribution)
    pub fn attribution(&self) -> FrozenArray<TaskAttributionTiming> {
        self.inner
            .get("attribution")
            .as_::<FrozenArray<TaskAttributionTiming>>()
    }
}
impl PerformanceLongTaskTiming {
    /// The toJSON method.
    /// [`PerformanceLongTaskTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceLongTaskTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
