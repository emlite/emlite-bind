use super::*;




/// The GPUQuerySetDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQuerySetDescriptor {
    inner: Any,
}

impl FromVal for GPUQuerySetDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUQuerySetDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUQuerySetDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUQuerySetDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUQuerySetDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUQuerySetDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUQuerySetDescriptor> for Any {
    fn from(s: GPUQuerySetDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUQuerySetDescriptor> for Any {
    fn from(s: &GPUQuerySetDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUQuerySetDescriptor {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> GPUQueryType {
        self.inner.get("type").as_::<GPUQueryType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &GPUQueryType) {
        self.inner.set("type", value);
    }
}
impl GPUQuerySetDescriptor {
    /// Getter of the `count` attribute.
    pub fn count(&self) -> Any {
        self.inner.get("count").as_::<Any>()
    }

    /// Setter of the `count` attribute.
    pub fn set_count(&mut self, value: &Any) {
        self.inner.set("count", value);
    }
}
