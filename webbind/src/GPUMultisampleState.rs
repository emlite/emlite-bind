use super::*;




/// The GPUMultisampleState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUMultisampleState {
    inner: Any,
}

impl FromVal for GPUMultisampleState {
    fn from_val(v: &Any) -> Self {
        GPUMultisampleState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUMultisampleState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUMultisampleState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUMultisampleState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUMultisampleState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUMultisampleState> for Any {
    fn from(s: GPUMultisampleState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUMultisampleState> for Any {
    fn from(s: &GPUMultisampleState) -> Any {
        s.inner.clone()
    }
}

impl GPUMultisampleState {
    /// Getter of the `count` attribute.
    pub fn count(&self) -> Any {
        self.inner.get("count").as_::<Any>()
    }

    /// Setter of the `count` attribute.
    pub fn set_count(&mut self, value: &Any) {
        self.inner.set("count", value);
    }
}
impl GPUMultisampleState {
    /// Getter of the `mask` attribute.
    pub fn mask(&self) -> Any {
        self.inner.get("mask").as_::<Any>()
    }

    /// Setter of the `mask` attribute.
    pub fn set_mask(&mut self, value: &Any) {
        self.inner.set("mask", value);
    }
}
impl GPUMultisampleState {
    /// Getter of the `alphaToCoverageEnabled` attribute.
    pub fn alpha_to_coverage_enabled(&self) -> bool {
        self.inner.get("alphaToCoverageEnabled").as_::<bool>()
    }

    /// Setter of the `alphaToCoverageEnabled` attribute.
    pub fn set_alpha_to_coverage_enabled(&mut self, value: bool) {
        self.inner.set("alphaToCoverageEnabled", value);
    }
}
