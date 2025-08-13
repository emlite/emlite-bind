use super::*;




/// The PerformanceMarkOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMarkOptions {
    inner: Any,
}

impl FromVal for PerformanceMarkOptions {
    fn from_val(v: &Any) -> Self {
        PerformanceMarkOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceMarkOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceMarkOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceMarkOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceMarkOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PerformanceMarkOptions> for Any {
    fn from(s: PerformanceMarkOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceMarkOptions> for Any {
    fn from(s: &PerformanceMarkOptions) -> Any {
        s.inner.clone()
    }
}

impl PerformanceMarkOptions {
    /// Getter of the `detail` attribute.
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }

    /// Setter of the `detail` attribute.
    pub fn set_detail(&mut self, value: &Any) {
        self.inner.set("detail", value);
    }
}
impl PerformanceMarkOptions {
    /// Getter of the `startTime` attribute.
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }

    /// Setter of the `startTime` attribute.
    pub fn set_start_time(&mut self, value: &Any) {
        self.inner.set("startTime", value);
    }
}
