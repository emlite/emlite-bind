use super::*;

/// The GPUPipelineDescriptorBase dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineDescriptorBase {
    inner: Any,
}

impl FromVal for GPUPipelineDescriptorBase {
    fn from_val(v: &Any) -> Self {
        GPUPipelineDescriptorBase { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUPipelineDescriptorBase {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUPipelineDescriptorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUPipelineDescriptorBase {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUPipelineDescriptorBase {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUPipelineDescriptorBase> for Any {
    fn from(s: GPUPipelineDescriptorBase) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUPipelineDescriptorBase> for Any {
    fn from(s: &GPUPipelineDescriptorBase) -> Any {
        s.inner.clone()
    }
}

impl GPUPipelineDescriptorBase {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> Any {
        self.inner.get("layout").as_::<Any>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &Any) {
        self.inner.set("layout", value);
    }
}
