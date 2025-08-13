use super::*;




/// The PerformanceObserverCallbackOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserverCallbackOptions {
    inner: Any,
}

impl FromVal for PerformanceObserverCallbackOptions {
    fn from_val(v: &Any) -> Self {
        PerformanceObserverCallbackOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceObserverCallbackOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceObserverCallbackOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceObserverCallbackOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceObserverCallbackOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PerformanceObserverCallbackOptions> for Any {
    fn from(s: PerformanceObserverCallbackOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceObserverCallbackOptions> for Any {
    fn from(s: &PerformanceObserverCallbackOptions) -> Any {
        s.inner.clone()
    }
}

impl PerformanceObserverCallbackOptions {
    /// Getter of the `droppedEntriesCount` attribute.
    pub fn dropped_entries_count(&self) -> u64 {
        self.inner.get("droppedEntriesCount").as_::<u64>()
    }

    /// Setter of the `droppedEntriesCount` attribute.
    pub fn set_dropped_entries_count(&mut self, value: u64) {
        self.inner.set("droppedEntriesCount", value);
    }
}
