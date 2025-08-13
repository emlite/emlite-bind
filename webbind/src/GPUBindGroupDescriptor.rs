use super::*;




/// The GPUBindGroupDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupDescriptor {
    inner: Any,
}

impl FromVal for GPUBindGroupDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBindGroupDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBindGroupDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBindGroupDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBindGroupDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUBindGroupDescriptor> for Any {
    fn from(s: GPUBindGroupDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBindGroupDescriptor> for Any {
    fn from(s: &GPUBindGroupDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupDescriptor {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> GPUBindGroupLayout {
        self.inner.get("layout").as_::<GPUBindGroupLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &GPUBindGroupLayout) {
        self.inner.set("layout", value);
    }
}
impl GPUBindGroupDescriptor {
    /// Getter of the `entries` attribute.
    pub fn entries(&self) -> TypedArray<GPUBindGroupEntry> {
        self.inner.get("entries").as_::<TypedArray<GPUBindGroupEntry>>()
    }

    /// Setter of the `entries` attribute.
    pub fn set_entries(&mut self, value: &TypedArray<GPUBindGroupEntry>) {
        self.inner.set("entries", value);
    }
}
