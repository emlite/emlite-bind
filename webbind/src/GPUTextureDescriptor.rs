use super::*;




/// The GPUTextureDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTextureDescriptor {
    inner: Any,
}

impl FromVal for GPUTextureDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTextureDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTextureDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTextureDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTextureDescriptor> for Any {
    fn from(s: GPUTextureDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTextureDescriptor> for Any {
    fn from(s: &GPUTextureDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUTextureDescriptor {
    /// Getter of the `size` attribute.
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }

    /// Setter of the `size` attribute.
    pub fn set_size(&mut self, value: &Any) {
        self.inner.set("size", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `mipLevelCount` attribute.
    pub fn mip_level_count(&self) -> Any {
        self.inner.get("mipLevelCount").as_::<Any>()
    }

    /// Setter of the `mipLevelCount` attribute.
    pub fn set_mip_level_count(&mut self, value: &Any) {
        self.inner.set("mipLevelCount", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `sampleCount` attribute.
    pub fn sample_count(&self) -> Any {
        self.inner.get("sampleCount").as_::<Any>()
    }

    /// Setter of the `sampleCount` attribute.
    pub fn set_sample_count(&mut self, value: &Any) {
        self.inner.set("sampleCount", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `dimension` attribute.
    pub fn dimension(&self) -> GPUTextureDimension {
        self.inner.get("dimension").as_::<GPUTextureDimension>()
    }

    /// Setter of the `dimension` attribute.
    pub fn set_dimension(&mut self, value: &GPUTextureDimension) {
        self.inner.set("dimension", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUTextureDescriptor {
    /// Getter of the `viewFormats` attribute.
    pub fn view_formats(&self) -> TypedArray<GPUTextureFormat> {
        self.inner.get("viewFormats").as_::<TypedArray<GPUTextureFormat>>()
    }

    /// Setter of the `viewFormats` attribute.
    pub fn set_view_formats(&mut self, value: &TypedArray<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }
}
