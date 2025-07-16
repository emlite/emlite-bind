use super::*;

/// The TimeRanges class.
/// [`TimeRanges`](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TimeRanges {
    inner: Any,
}
impl FromVal for TimeRanges {
    fn from_val(v: &Any) -> Self {
        TimeRanges {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TimeRanges {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TimeRanges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TimeRanges {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TimeRanges {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TimeRanges> for Any {
    fn from(s: TimeRanges) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TimeRanges> for Any {
    fn from(s: &TimeRanges) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TimeRanges);

impl TimeRanges {
    /// Getter of the `length` attribute.
    /// [`TimeRanges.length`](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TimeRanges {
    /// The start method.
    /// [`TimeRanges.start`](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)
    pub fn start(&self, index: u32) -> f64 {
        self.inner.call("start", &[index.into()]).as_::<f64>()
    }
}
impl TimeRanges {
    /// The end method.
    /// [`TimeRanges.end`](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)
    pub fn end(&self, index: u32) -> f64 {
        self.inner.call("end", &[index.into()]).as_::<f64>()
    }
}
