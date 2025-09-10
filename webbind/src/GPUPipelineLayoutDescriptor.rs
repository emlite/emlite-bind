use super::*;

/// The GPUPipelineLayoutDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineLayoutDescriptor {
    inner: Any,
}

impl FromVal for GPUPipelineLayoutDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUPipelineLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUPipelineLayoutDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUPipelineLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUPipelineLayoutDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUPipelineLayoutDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUPipelineLayoutDescriptor> for Any {
    fn from(s: GPUPipelineLayoutDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUPipelineLayoutDescriptor> for Any {
    fn from(s: &GPUPipelineLayoutDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUPipelineLayoutDescriptor {
    /// Getter of the `bindGroupLayouts` attribute.
    pub fn bind_group_layouts(&self) -> TypedArray<GPUBindGroupLayout> {
        self.inner
            .get("bindGroupLayouts")
            .as_::<TypedArray<GPUBindGroupLayout>>()
    }

    /// Setter of the `bindGroupLayouts` attribute.
    pub fn set_bind_group_layouts(&mut self, value: &TypedArray<GPUBindGroupLayout>) {
        self.inner.set("bindGroupLayouts", value);
    }
}
