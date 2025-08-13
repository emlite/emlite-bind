use super::*;




/// The GPUBufferDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBufferDescriptor {
    inner: Any,
}

impl FromVal for GPUBufferDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBufferDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBufferDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBufferDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBufferDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBufferDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUBufferDescriptor> for Any {
    fn from(s: GPUBufferDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBufferDescriptor> for Any {
    fn from(s: &GPUBufferDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBufferDescriptor {
    /// Getter of the `size` attribute.
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }

    /// Setter of the `size` attribute.
    pub fn set_size(&mut self, value: &Any) {
        self.inner.set("size", value);
    }
}
impl GPUBufferDescriptor {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUBufferDescriptor {
    /// Getter of the `mappedAtCreation` attribute.
    pub fn mapped_at_creation(&self) -> bool {
        self.inner.get("mappedAtCreation").as_::<bool>()
    }

    /// Setter of the `mappedAtCreation` attribute.
    pub fn set_mapped_at_creation(&mut self, value: bool) {
        self.inner.set("mappedAtCreation", value);
    }
}
