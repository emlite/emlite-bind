use super::*;




/// The GPUStorageTextureBindingLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUStorageTextureBindingLayout {
    inner: Any,
}

impl FromVal for GPUStorageTextureBindingLayout {
    fn from_val(v: &Any) -> Self {
        GPUStorageTextureBindingLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUStorageTextureBindingLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUStorageTextureBindingLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUStorageTextureBindingLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUStorageTextureBindingLayout {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUStorageTextureBindingLayout> for Any {
    fn from(s: GPUStorageTextureBindingLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUStorageTextureBindingLayout> for Any {
    fn from(s: &GPUStorageTextureBindingLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUStorageTextureBindingLayout {
    /// Getter of the `access` attribute.
    pub fn access(&self) -> GPUStorageTextureAccess {
        self.inner.get("access").as_::<GPUStorageTextureAccess>()
    }

    /// Setter of the `access` attribute.
    pub fn set_access(&mut self, value: &GPUStorageTextureAccess) {
        self.inner.set("access", value);
    }
}
impl GPUStorageTextureBindingLayout {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUStorageTextureBindingLayout {
    /// Getter of the `viewDimension` attribute.
    pub fn view_dimension(&self) -> GPUTextureViewDimension {
        self.inner.get("viewDimension").as_::<GPUTextureViewDimension>()
    }

    /// Setter of the `viewDimension` attribute.
    pub fn set_view_dimension(&mut self, value: &GPUTextureViewDimension) {
        self.inner.set("viewDimension", value);
    }
}
