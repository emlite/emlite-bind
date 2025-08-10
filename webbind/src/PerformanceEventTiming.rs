use super::*;

/// The PerformanceEventTiming class.
/// [`PerformanceEventTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceEventTiming {
    inner: PerformanceEntry,
}

impl FromVal for PerformanceEventTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceEventTiming {
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

impl AsRef<Any> for PerformanceEventTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceEventTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceEventTiming> for Any {
    fn from(s: PerformanceEventTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceEventTiming> for Any {
    fn from(s: &PerformanceEventTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceEventTiming);

impl PerformanceEventTiming {
    /// Getter of the `processingStart` attribute.
    /// [`PerformanceEventTiming.processingStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/processingStart)
    pub fn processing_start(&self) -> Any {
        self.inner.get("processingStart").as_::<Any>()
    }
}
impl PerformanceEventTiming {
    /// Getter of the `processingEnd` attribute.
    /// [`PerformanceEventTiming.processingEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/processingEnd)
    pub fn processing_end(&self) -> Any {
        self.inner.get("processingEnd").as_::<Any>()
    }
}
impl PerformanceEventTiming {
    /// Getter of the `cancelable` attribute.
    /// [`PerformanceEventTiming.cancelable`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/cancelable)
    pub fn cancelable(&self) -> bool {
        self.inner.get("cancelable").as_::<bool>()
    }
}
impl PerformanceEventTiming {
    /// Getter of the `target` attribute.
    /// [`PerformanceEventTiming.target`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/target)
    pub fn target(&self) -> Node {
        self.inner.get("target").as_::<Node>()
    }
}
impl PerformanceEventTiming {
    /// Getter of the `interactionId` attribute.
    /// [`PerformanceEventTiming.interactionId`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/interactionId)
    pub fn interaction_id(&self) -> u64 {
        self.inner.get("interactionId").as_::<u64>()
    }
}
impl PerformanceEventTiming {
    /// The toJSON method.
    /// [`PerformanceEventTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
