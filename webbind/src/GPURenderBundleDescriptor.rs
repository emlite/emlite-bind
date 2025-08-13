use super::*;




/// The GPURenderBundleDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundleDescriptor {
    inner: Any,
}

impl FromVal for GPURenderBundleDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderBundleDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderBundleDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderBundleDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderBundleDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderBundleDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPURenderBundleDescriptor> for Any {
    fn from(s: GPURenderBundleDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderBundleDescriptor> for Any {
    fn from(s: &GPURenderBundleDescriptor) -> Any {
        s.inner.clone()
    }
}

