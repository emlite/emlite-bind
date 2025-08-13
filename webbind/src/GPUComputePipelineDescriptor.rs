use super::*;




/// The GPUComputePipelineDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePipelineDescriptor {
    inner: Any,
}

impl FromVal for GPUComputePipelineDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUComputePipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUComputePipelineDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUComputePipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUComputePipelineDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUComputePipelineDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUComputePipelineDescriptor> for Any {
    fn from(s: GPUComputePipelineDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUComputePipelineDescriptor> for Any {
    fn from(s: &GPUComputePipelineDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUComputePipelineDescriptor {
    /// Getter of the `compute` attribute.
    pub fn compute(&self) -> GPUProgrammableStage {
        self.inner.get("compute").as_::<GPUProgrammableStage>()
    }

    /// Setter of the `compute` attribute.
    pub fn set_compute(&mut self, value: &GPUProgrammableStage) {
        self.inner.set("compute", value);
    }
}
