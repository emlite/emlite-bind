use super::*;




/// The GPUColorTargetState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUColorTargetState {
    inner: Any,
}

impl FromVal for GPUColorTargetState {
    fn from_val(v: &Any) -> Self {
        GPUColorTargetState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUColorTargetState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUColorTargetState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUColorTargetState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUColorTargetState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUColorTargetState> for Any {
    fn from(s: GPUColorTargetState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUColorTargetState> for Any {
    fn from(s: &GPUColorTargetState) -> Any {
        s.inner.clone()
    }
}

impl GPUColorTargetState {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUColorTargetState {
    /// Getter of the `blend` attribute.
    pub fn blend(&self) -> GPUBlendState {
        self.inner.get("blend").as_::<GPUBlendState>()
    }

    /// Setter of the `blend` attribute.
    pub fn set_blend(&mut self, value: &GPUBlendState) {
        self.inner.set("blend", value);
    }
}
impl GPUColorTargetState {
    /// Getter of the `writeMask` attribute.
    pub fn write_mask(&self) -> Any {
        self.inner.get("writeMask").as_::<Any>()
    }

    /// Setter of the `writeMask` attribute.
    pub fn set_write_mask(&mut self, value: &Any) {
        self.inner.set("writeMask", value);
    }
}
