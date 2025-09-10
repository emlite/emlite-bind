use super::*;

/// The GPUBindGroupLayoutDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupLayoutDescriptor {
    inner: Any,
}

impl FromVal for GPUBindGroupLayoutDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBindGroupLayoutDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBindGroupLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBindGroupLayoutDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBindGroupLayoutDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUBindGroupLayoutDescriptor> for Any {
    fn from(s: GPUBindGroupLayoutDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBindGroupLayoutDescriptor> for Any {
    fn from(s: &GPUBindGroupLayoutDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupLayoutDescriptor {
    /// Getter of the `entries` attribute.
    pub fn entries(&self) -> TypedArray<GPUBindGroupLayoutEntry> {
        self.inner
            .get("entries")
            .as_::<TypedArray<GPUBindGroupLayoutEntry>>()
    }

    /// Setter of the `entries` attribute.
    pub fn set_entries(&mut self, value: &TypedArray<GPUBindGroupLayoutEntry>) {
        self.inner.set("entries", value);
    }
}
