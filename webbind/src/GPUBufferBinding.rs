use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBufferBinding {
    inner: Any,
}
impl FromVal for GPUBufferBinding {
    fn from_val(v: &Any) -> Self {
        GPUBufferBinding { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBufferBinding {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBufferBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBufferBinding {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBufferBinding {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBufferBinding> for Any {
    fn from(s: GPUBufferBinding) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBufferBinding> for Any {
    fn from(s: &GPUBufferBinding) -> Any {
        s.inner.clone()
    }
}

impl GPUBufferBinding {
    pub fn buffer(&self) -> GPUBuffer {
        self.inner.get("buffer").as_::<GPUBuffer>()
    }

    pub fn set_buffer(&mut self, value: &GPUBuffer) {
        self.inner.set("buffer", value);
    }
}
impl GPUBufferBinding {
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
impl GPUBufferBinding {
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }

    pub fn set_size(&mut self, value: &Any) {
        self.inner.set("size", value);
    }
}
