use super::*;




/// The GPUTexelCopyBufferLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferLayout {
    inner: Any,
}

impl FromVal for GPUTexelCopyBufferLayout {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyBufferLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUTexelCopyBufferLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTexelCopyBufferLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTexelCopyBufferLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTexelCopyBufferLayout {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUTexelCopyBufferLayout> for Any {
    fn from(s: GPUTexelCopyBufferLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTexelCopyBufferLayout> for Any {
    fn from(s: &GPUTexelCopyBufferLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyBufferLayout {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
impl GPUTexelCopyBufferLayout {
    /// Getter of the `bytesPerRow` attribute.
    pub fn bytes_per_row(&self) -> Any {
        self.inner.get("bytesPerRow").as_::<Any>()
    }

    /// Setter of the `bytesPerRow` attribute.
    pub fn set_bytes_per_row(&mut self, value: &Any) {
        self.inner.set("bytesPerRow", value);
    }
}
impl GPUTexelCopyBufferLayout {
    /// Getter of the `rowsPerImage` attribute.
    pub fn rows_per_image(&self) -> Any {
        self.inner.get("rowsPerImage").as_::<Any>()
    }

    /// Setter of the `rowsPerImage` attribute.
    pub fn set_rows_per_image(&mut self, value: &Any) {
        self.inner.set("rowsPerImage", value);
    }
}
