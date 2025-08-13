use super::*;




/// The GPUSamplerDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSamplerDescriptor {
    inner: Any,
}

impl FromVal for GPUSamplerDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUSamplerDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUSamplerDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUSamplerDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUSamplerDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUSamplerDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUSamplerDescriptor> for Any {
    fn from(s: GPUSamplerDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUSamplerDescriptor> for Any {
    fn from(s: &GPUSamplerDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUSamplerDescriptor {
    /// Getter of the `addressModeU` attribute.
    pub fn address_mode_u(&self) -> GPUAddressMode {
        self.inner.get("addressModeU").as_::<GPUAddressMode>()
    }

    /// Setter of the `addressModeU` attribute.
    pub fn set_address_mode_u(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeU", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `addressModeV` attribute.
    pub fn address_mode_v(&self) -> GPUAddressMode {
        self.inner.get("addressModeV").as_::<GPUAddressMode>()
    }

    /// Setter of the `addressModeV` attribute.
    pub fn set_address_mode_v(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeV", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `addressModeW` attribute.
    pub fn address_mode_w(&self) -> GPUAddressMode {
        self.inner.get("addressModeW").as_::<GPUAddressMode>()
    }

    /// Setter of the `addressModeW` attribute.
    pub fn set_address_mode_w(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeW", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `magFilter` attribute.
    pub fn mag_filter(&self) -> GPUFilterMode {
        self.inner.get("magFilter").as_::<GPUFilterMode>()
    }

    /// Setter of the `magFilter` attribute.
    pub fn set_mag_filter(&mut self, value: &GPUFilterMode) {
        self.inner.set("magFilter", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `minFilter` attribute.
    pub fn min_filter(&self) -> GPUFilterMode {
        self.inner.get("minFilter").as_::<GPUFilterMode>()
    }

    /// Setter of the `minFilter` attribute.
    pub fn set_min_filter(&mut self, value: &GPUFilterMode) {
        self.inner.set("minFilter", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `mipmapFilter` attribute.
    pub fn mipmap_filter(&self) -> GPUMipmapFilterMode {
        self.inner.get("mipmapFilter").as_::<GPUMipmapFilterMode>()
    }

    /// Setter of the `mipmapFilter` attribute.
    pub fn set_mipmap_filter(&mut self, value: &GPUMipmapFilterMode) {
        self.inner.set("mipmapFilter", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `lodMinClamp` attribute.
    pub fn lod_min_clamp(&self) -> f32 {
        self.inner.get("lodMinClamp").as_::<f32>()
    }

    /// Setter of the `lodMinClamp` attribute.
    pub fn set_lod_min_clamp(&mut self, value: f32) {
        self.inner.set("lodMinClamp", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `lodMaxClamp` attribute.
    pub fn lod_max_clamp(&self) -> f32 {
        self.inner.get("lodMaxClamp").as_::<f32>()
    }

    /// Setter of the `lodMaxClamp` attribute.
    pub fn set_lod_max_clamp(&mut self, value: f32) {
        self.inner.set("lodMaxClamp", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `compare` attribute.
    pub fn compare(&self) -> GPUCompareFunction {
        self.inner.get("compare").as_::<GPUCompareFunction>()
    }

    /// Setter of the `compare` attribute.
    pub fn set_compare(&mut self, value: &GPUCompareFunction) {
        self.inner.set("compare", value);
    }
}
impl GPUSamplerDescriptor {
    /// Getter of the `maxAnisotropy` attribute.
    pub fn max_anisotropy(&self) -> u16 {
        self.inner.get("maxAnisotropy").as_::<u16>()
    }

    /// Setter of the `maxAnisotropy` attribute.
    pub fn set_max_anisotropy(&mut self, value: u16) {
        self.inner.set("maxAnisotropy", value);
    }
}
