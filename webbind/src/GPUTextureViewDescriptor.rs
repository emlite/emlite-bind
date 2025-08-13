use super::*;




/// The GPUTextureViewDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTextureViewDescriptor {
    inner: Any,
}

impl FromVal for GPUTextureViewDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUTextureViewDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTextureViewDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTextureViewDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTextureViewDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTextureViewDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTextureViewDescriptor> for Any {
    fn from(s: GPUTextureViewDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTextureViewDescriptor> for Any {
    fn from(s: &GPUTextureViewDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUTextureViewDescriptor {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `dimension` attribute.
    pub fn dimension(&self) -> GPUTextureViewDimension {
        self.inner.get("dimension").as_::<GPUTextureViewDimension>()
    }

    /// Setter of the `dimension` attribute.
    pub fn set_dimension(&mut self, value: &GPUTextureViewDimension) {
        self.inner.set("dimension", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `aspect` attribute.
    pub fn aspect(&self) -> GPUTextureAspect {
        self.inner.get("aspect").as_::<GPUTextureAspect>()
    }

    /// Setter of the `aspect` attribute.
    pub fn set_aspect(&mut self, value: &GPUTextureAspect) {
        self.inner.set("aspect", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `baseMipLevel` attribute.
    pub fn base_mip_level(&self) -> Any {
        self.inner.get("baseMipLevel").as_::<Any>()
    }

    /// Setter of the `baseMipLevel` attribute.
    pub fn set_base_mip_level(&mut self, value: &Any) {
        self.inner.set("baseMipLevel", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `mipLevelCount` attribute.
    pub fn mip_level_count(&self) -> Any {
        self.inner.get("mipLevelCount").as_::<Any>()
    }

    /// Setter of the `mipLevelCount` attribute.
    pub fn set_mip_level_count(&mut self, value: &Any) {
        self.inner.set("mipLevelCount", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `baseArrayLayer` attribute.
    pub fn base_array_layer(&self) -> Any {
        self.inner.get("baseArrayLayer").as_::<Any>()
    }

    /// Setter of the `baseArrayLayer` attribute.
    pub fn set_base_array_layer(&mut self, value: &Any) {
        self.inner.set("baseArrayLayer", value);
    }
}
impl GPUTextureViewDescriptor {
    /// Getter of the `arrayLayerCount` attribute.
    pub fn array_layer_count(&self) -> Any {
        self.inner.get("arrayLayerCount").as_::<Any>()
    }

    /// Setter of the `arrayLayerCount` attribute.
    pub fn set_array_layer_count(&mut self, value: &Any) {
        self.inner.set("arrayLayerCount", value);
    }
}
