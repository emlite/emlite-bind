use super::*;

/// The PerformancePaintTiming class.
/// [`PerformancePaintTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformancePaintTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformancePaintTiming {
    inner: PerformanceEntry,
}

impl FromVal for PerformancePaintTiming {
    fn from_val(v: &Any) -> Self {
        PerformancePaintTiming {
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

impl core::ops::Deref for PerformancePaintTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformancePaintTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformancePaintTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformancePaintTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformancePaintTiming> for Any {
    fn from(s: PerformancePaintTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformancePaintTiming> for Any {
    fn from(s: &PerformancePaintTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformancePaintTiming);

impl PerformancePaintTiming {
    /// Getter of the `paintTime` attribute.
    /// [`PerformancePaintTiming.paintTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformancePaintTiming/paintTime)
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }
}
impl PerformancePaintTiming {
    /// Getter of the `presentationTime` attribute.
    /// [`PerformancePaintTiming.presentationTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformancePaintTiming/presentationTime)
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }
}
impl PerformancePaintTiming {
    /// The toJSON method.
    /// [`PerformancePaintTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformancePaintTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
