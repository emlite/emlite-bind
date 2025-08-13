use super::*;




/// The GPUBindGroupEntry dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupEntry {
    inner: Any,
}

impl FromVal for GPUBindGroupEntry {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupEntry { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBindGroupEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBindGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBindGroupEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBindGroupEntry {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUBindGroupEntry> for Any {
    fn from(s: GPUBindGroupEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBindGroupEntry> for Any {
    fn from(s: &GPUBindGroupEntry) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupEntry {
    /// Getter of the `binding` attribute.
    pub fn binding(&self) -> Any {
        self.inner.get("binding").as_::<Any>()
    }

    /// Setter of the `binding` attribute.
    pub fn set_binding(&mut self, value: &Any) {
        self.inner.set("binding", value);
    }
}
impl GPUBindGroupEntry {
    /// Getter of the `resource` attribute.
    pub fn resource(&self) -> Any {
        self.inner.get("resource").as_::<Any>()
    }

    /// Setter of the `resource` attribute.
    pub fn set_resource(&mut self, value: &Any) {
        self.inner.set("resource", value);
    }
}
