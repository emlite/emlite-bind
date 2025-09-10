use super::*;

/// The GPURenderPipelineDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPipelineDescriptor {
    inner: Any,
}

impl FromVal for GPURenderPipelineDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderPipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderPipelineDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderPipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderPipelineDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderPipelineDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderPipelineDescriptor> for Any {
    fn from(s: GPURenderPipelineDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderPipelineDescriptor> for Any {
    fn from(s: &GPURenderPipelineDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPipelineDescriptor {
    /// Getter of the `vertex` attribute.
    pub fn vertex(&self) -> GPUVertexState {
        self.inner.get("vertex").as_::<GPUVertexState>()
    }

    /// Setter of the `vertex` attribute.
    pub fn set_vertex(&mut self, value: &GPUVertexState) {
        self.inner.set("vertex", value);
    }
}
impl GPURenderPipelineDescriptor {
    /// Getter of the `primitive` attribute.
    pub fn primitive(&self) -> GPUPrimitiveState {
        self.inner.get("primitive").as_::<GPUPrimitiveState>()
    }

    /// Setter of the `primitive` attribute.
    pub fn set_primitive(&mut self, value: &GPUPrimitiveState) {
        self.inner.set("primitive", value);
    }
}
impl GPURenderPipelineDescriptor {
    /// Getter of the `depthStencil` attribute.
    pub fn depth_stencil(&self) -> GPUDepthStencilState {
        self.inner.get("depthStencil").as_::<GPUDepthStencilState>()
    }

    /// Setter of the `depthStencil` attribute.
    pub fn set_depth_stencil(&mut self, value: &GPUDepthStencilState) {
        self.inner.set("depthStencil", value);
    }
}
impl GPURenderPipelineDescriptor {
    /// Getter of the `multisample` attribute.
    pub fn multisample(&self) -> GPUMultisampleState {
        self.inner.get("multisample").as_::<GPUMultisampleState>()
    }

    /// Setter of the `multisample` attribute.
    pub fn set_multisample(&mut self, value: &GPUMultisampleState) {
        self.inner.set("multisample", value);
    }
}
impl GPURenderPipelineDescriptor {
    /// Getter of the `fragment` attribute.
    pub fn fragment(&self) -> GPUFragmentState {
        self.inner.get("fragment").as_::<GPUFragmentState>()
    }

    /// Setter of the `fragment` attribute.
    pub fn set_fragment(&mut self, value: &GPUFragmentState) {
        self.inner.set("fragment", value);
    }
}
