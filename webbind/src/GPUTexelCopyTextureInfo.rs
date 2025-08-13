use super::*;




/// The GPUTexelCopyTextureInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyTextureInfo {
    inner: Any,
}

impl FromVal for GPUTexelCopyTextureInfo {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyTextureInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTexelCopyTextureInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTexelCopyTextureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTexelCopyTextureInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTexelCopyTextureInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTexelCopyTextureInfo> for Any {
    fn from(s: GPUTexelCopyTextureInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTexelCopyTextureInfo> for Any {
    fn from(s: &GPUTexelCopyTextureInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyTextureInfo {
    /// Getter of the `texture` attribute.
    pub fn texture(&self) -> GPUTexture {
        self.inner.get("texture").as_::<GPUTexture>()
    }

    /// Setter of the `texture` attribute.
    pub fn set_texture(&mut self, value: &GPUTexture) {
        self.inner.set("texture", value);
    }
}
impl GPUTexelCopyTextureInfo {
    /// Getter of the `mipLevel` attribute.
    pub fn mip_level(&self) -> Any {
        self.inner.get("mipLevel").as_::<Any>()
    }

    /// Setter of the `mipLevel` attribute.
    pub fn set_mip_level(&mut self, value: &Any) {
        self.inner.set("mipLevel", value);
    }
}
impl GPUTexelCopyTextureInfo {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> Any {
        self.inner.get("origin").as_::<Any>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &Any) {
        self.inner.set("origin", value);
    }
}
impl GPUTexelCopyTextureInfo {
    /// Getter of the `aspect` attribute.
    pub fn aspect(&self) -> GPUTextureAspect {
        self.inner.get("aspect").as_::<GPUTextureAspect>()
    }

    /// Setter of the `aspect` attribute.
    pub fn set_aspect(&mut self, value: &GPUTextureAspect) {
        self.inner.set("aspect", value);
    }
}
