use super::*;

/// The TaskAttributionTiming class.
/// [`TaskAttributionTiming`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskAttributionTiming {
    inner: PerformanceEntry,
}
impl FromVal for TaskAttributionTiming {
    fn from_val(v: &Any) -> Self {
        TaskAttributionTiming {
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
impl core::ops::Deref for TaskAttributionTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskAttributionTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TaskAttributionTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TaskAttributionTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TaskAttributionTiming> for Any {
    fn from(s: TaskAttributionTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TaskAttributionTiming> for Any {
    fn from(s: &TaskAttributionTiming) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TaskAttributionTiming);

impl TaskAttributionTiming {
    /// Getter of the `startTime` attribute.
    /// [`TaskAttributionTiming.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `duration` attribute.
    /// [`TaskAttributionTiming.duration`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `name` attribute.
    /// [`TaskAttributionTiming.name`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `entryType` attribute.
    /// [`TaskAttributionTiming.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/entryType)
    pub fn entry_type(&self) -> DOMString {
        self.inner.get("entryType").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `containerType` attribute.
    /// [`TaskAttributionTiming.containerType`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/containerType)
    pub fn container_type(&self) -> DOMString {
        self.inner.get("containerType").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `containerSrc` attribute.
    /// [`TaskAttributionTiming.containerSrc`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/containerSrc)
    pub fn container_src(&self) -> DOMString {
        self.inner.get("containerSrc").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `containerId` attribute.
    /// [`TaskAttributionTiming.containerId`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/containerId)
    pub fn container_id(&self) -> DOMString {
        self.inner.get("containerId").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// Getter of the `containerName` attribute.
    /// [`TaskAttributionTiming.containerName`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/containerName)
    pub fn container_name(&self) -> DOMString {
        self.inner.get("containerName").as_::<DOMString>()
    }
}
impl TaskAttributionTiming {
    /// The toJSON method.
    /// [`TaskAttributionTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/TaskAttributionTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
