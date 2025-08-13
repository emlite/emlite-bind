use super::*;




/// The GPUCommandEncoderDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandEncoderDescriptor {
    inner: Any,
}

impl FromVal for GPUCommandEncoderDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUCommandEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUCommandEncoderDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUCommandEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUCommandEncoderDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUCommandEncoderDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUCommandEncoderDescriptor> for Any {
    fn from(s: GPUCommandEncoderDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUCommandEncoderDescriptor> for Any {
    fn from(s: &GPUCommandEncoderDescriptor) -> Any {
        s.inner.clone()
    }
}

