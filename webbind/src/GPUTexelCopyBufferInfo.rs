use super::*;




/// The GPUTexelCopyBufferInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferInfo {
    inner: Any,
}

impl FromVal for GPUTexelCopyBufferInfo {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyBufferInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTexelCopyBufferInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTexelCopyBufferInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTexelCopyBufferInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTexelCopyBufferInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTexelCopyBufferInfo> for Any {
    fn from(s: GPUTexelCopyBufferInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTexelCopyBufferInfo> for Any {
    fn from(s: &GPUTexelCopyBufferInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyBufferInfo {
    /// Getter of the `buffer` attribute.
    pub fn buffer(&self) -> GPUBuffer {
        self.inner.get("buffer").as_::<GPUBuffer>()
    }

    /// Setter of the `buffer` attribute.
    pub fn set_buffer(&mut self, value: &GPUBuffer) {
        self.inner.set("buffer", value);
    }
}
