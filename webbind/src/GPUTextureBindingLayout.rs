use super::*;




/// The GPUTextureBindingLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTextureBindingLayout {
    inner: Any,
}

impl FromVal for GPUTextureBindingLayout {
    fn from_val(v: &Any) -> Self {
        GPUTextureBindingLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTextureBindingLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTextureBindingLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTextureBindingLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTextureBindingLayout {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTextureBindingLayout> for Any {
    fn from(s: GPUTextureBindingLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTextureBindingLayout> for Any {
    fn from(s: &GPUTextureBindingLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUTextureBindingLayout {
    /// Getter of the `sampleType` attribute.
    pub fn sample_type(&self) -> GPUTextureSampleType {
        self.inner.get("sampleType").as_::<GPUTextureSampleType>()
    }

    /// Setter of the `sampleType` attribute.
    pub fn set_sample_type(&mut self, value: &GPUTextureSampleType) {
        self.inner.set("sampleType", value);
    }
}
impl GPUTextureBindingLayout {
    /// Getter of the `viewDimension` attribute.
    pub fn view_dimension(&self) -> GPUTextureViewDimension {
        self.inner.get("viewDimension").as_::<GPUTextureViewDimension>()
    }

    /// Setter of the `viewDimension` attribute.
    pub fn set_view_dimension(&mut self, value: &GPUTextureViewDimension) {
        self.inner.set("viewDimension", value);
    }
}
impl GPUTextureBindingLayout {
    /// Getter of the `multisampled` attribute.
    pub fn multisampled(&self) -> bool {
        self.inner.get("multisampled").as_::<bool>()
    }

    /// Setter of the `multisampled` attribute.
    pub fn set_multisampled(&mut self, value: bool) {
        self.inner.set("multisampled", value);
    }
}
