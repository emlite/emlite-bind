use super::*;




/// The GPURenderPassDepthStencilAttachment dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassDepthStencilAttachment {
    inner: Any,
}

impl FromVal for GPURenderPassDepthStencilAttachment {
    fn from_val(v: &Any) -> Self {
        GPURenderPassDepthStencilAttachment { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderPassDepthStencilAttachment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderPassDepthStencilAttachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderPassDepthStencilAttachment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderPassDepthStencilAttachment {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPURenderPassDepthStencilAttachment> for Any {
    fn from(s: GPURenderPassDepthStencilAttachment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderPassDepthStencilAttachment> for Any {
    fn from(s: &GPURenderPassDepthStencilAttachment) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `view` attribute.
    pub fn view(&self) -> Any {
        self.inner.get("view").as_::<Any>()
    }

    /// Setter of the `view` attribute.
    pub fn set_view(&mut self, value: &Any) {
        self.inner.set("view", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `depthClearValue` attribute.
    pub fn depth_clear_value(&self) -> f32 {
        self.inner.get("depthClearValue").as_::<f32>()
    }

    /// Setter of the `depthClearValue` attribute.
    pub fn set_depth_clear_value(&mut self, value: f32) {
        self.inner.set("depthClearValue", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `depthLoadOp` attribute.
    pub fn depth_load_op(&self) -> GPULoadOp {
        self.inner.get("depthLoadOp").as_::<GPULoadOp>()
    }

    /// Setter of the `depthLoadOp` attribute.
    pub fn set_depth_load_op(&mut self, value: &GPULoadOp) {
        self.inner.set("depthLoadOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `depthStoreOp` attribute.
    pub fn depth_store_op(&self) -> GPUStoreOp {
        self.inner.get("depthStoreOp").as_::<GPUStoreOp>()
    }

    /// Setter of the `depthStoreOp` attribute.
    pub fn set_depth_store_op(&mut self, value: &GPUStoreOp) {
        self.inner.set("depthStoreOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `depthReadOnly` attribute.
    pub fn depth_read_only(&self) -> bool {
        self.inner.get("depthReadOnly").as_::<bool>()
    }

    /// Setter of the `depthReadOnly` attribute.
    pub fn set_depth_read_only(&mut self, value: bool) {
        self.inner.set("depthReadOnly", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `stencilClearValue` attribute.
    pub fn stencil_clear_value(&self) -> Any {
        self.inner.get("stencilClearValue").as_::<Any>()
    }

    /// Setter of the `stencilClearValue` attribute.
    pub fn set_stencil_clear_value(&mut self, value: &Any) {
        self.inner.set("stencilClearValue", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `stencilLoadOp` attribute.
    pub fn stencil_load_op(&self) -> GPULoadOp {
        self.inner.get("stencilLoadOp").as_::<GPULoadOp>()
    }

    /// Setter of the `stencilLoadOp` attribute.
    pub fn set_stencil_load_op(&mut self, value: &GPULoadOp) {
        self.inner.set("stencilLoadOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `stencilStoreOp` attribute.
    pub fn stencil_store_op(&self) -> GPUStoreOp {
        self.inner.get("stencilStoreOp").as_::<GPUStoreOp>()
    }

    /// Setter of the `stencilStoreOp` attribute.
    pub fn set_stencil_store_op(&mut self, value: &GPUStoreOp) {
        self.inner.set("stencilStoreOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    /// Getter of the `stencilReadOnly` attribute.
    pub fn stencil_read_only(&self) -> bool {
        self.inner.get("stencilReadOnly").as_::<bool>()
    }

    /// Setter of the `stencilReadOnly` attribute.
    pub fn set_stencil_read_only(&mut self, value: bool) {
        self.inner.set("stencilReadOnly", value);
    }
}
