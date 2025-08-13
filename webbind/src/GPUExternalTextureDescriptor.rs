use super::*;




/// The GPUExternalTextureDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUExternalTextureDescriptor {
    inner: Any,
}

impl FromVal for GPUExternalTextureDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUExternalTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUExternalTextureDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUExternalTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUExternalTextureDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUExternalTextureDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUExternalTextureDescriptor> for Any {
    fn from(s: GPUExternalTextureDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUExternalTextureDescriptor> for Any {
    fn from(s: &GPUExternalTextureDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUExternalTextureDescriptor {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
impl GPUExternalTextureDescriptor {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
