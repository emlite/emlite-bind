use super::*;

/// The LayoutShift class.
/// [`LayoutShift`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutShift {
    inner: PerformanceEntry,
}

impl FromVal for LayoutShift {
    fn from_val(v: &Any) -> Self {
        LayoutShift {
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

impl AsRef<Any> for LayoutShift {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LayoutShift {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<LayoutShift> for Any {
    fn from(s: LayoutShift) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LayoutShift> for Any {
    fn from(s: &LayoutShift) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LayoutShift);

impl LayoutShift {
    /// Getter of the `value` attribute.
    /// [`LayoutShift.value`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
impl LayoutShift {
    /// Getter of the `hadRecentInput` attribute.
    /// [`LayoutShift.hadRecentInput`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift/hadRecentInput)
    pub fn had_recent_input(&self) -> bool {
        self.inner.get("hadRecentInput").as_::<bool>()
    }
}
impl LayoutShift {
    /// Getter of the `lastInputTime` attribute.
    /// [`LayoutShift.lastInputTime`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift/lastInputTime)
    pub fn last_input_time(&self) -> Any {
        self.inner.get("lastInputTime").as_::<Any>()
    }
}
impl LayoutShift {
    /// Getter of the `sources` attribute.
    /// [`LayoutShift.sources`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift/sources)
    pub fn sources(&self) -> TypedArray<LayoutShiftAttribution> {
        self.inner
            .get("sources")
            .as_::<TypedArray<LayoutShiftAttribution>>()
    }
}
impl LayoutShift {
    /// The toJSON method.
    /// [`LayoutShift.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShift/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
