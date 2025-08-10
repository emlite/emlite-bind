use super::*;

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
    pub fn view(&self) -> GPUTextureView {
        self.inner.get("view").as_::<GPUTextureView>()
    }

    pub fn set_view(&mut self, value: &GPUTextureView) {
        self.inner.set("view", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn depth_clear_value(&self) -> f32 {
        self.inner.get("depthClearValue").as_::<f32>()
    }

    pub fn set_depth_clear_value(&mut self, value: f32) {
        self.inner.set("depthClearValue", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn depth_load_op(&self) -> GPULoadOp {
        self.inner.get("depthLoadOp").as_::<GPULoadOp>()
    }

    pub fn set_depth_load_op(&mut self, value: &GPULoadOp) {
        self.inner.set("depthLoadOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn depth_store_op(&self) -> GPUStoreOp {
        self.inner.get("depthStoreOp").as_::<GPUStoreOp>()
    }

    pub fn set_depth_store_op(&mut self, value: &GPUStoreOp) {
        self.inner.set("depthStoreOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn depth_read_only(&self) -> bool {
        self.inner.get("depthReadOnly").as_::<bool>()
    }

    pub fn set_depth_read_only(&mut self, value: bool) {
        self.inner.set("depthReadOnly", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn stencil_clear_value(&self) -> Any {
        self.inner.get("stencilClearValue").as_::<Any>()
    }

    pub fn set_stencil_clear_value(&mut self, value: &Any) {
        self.inner.set("stencilClearValue", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn stencil_load_op(&self) -> GPULoadOp {
        self.inner.get("stencilLoadOp").as_::<GPULoadOp>()
    }

    pub fn set_stencil_load_op(&mut self, value: &GPULoadOp) {
        self.inner.set("stencilLoadOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn stencil_store_op(&self) -> GPUStoreOp {
        self.inner.get("stencilStoreOp").as_::<GPUStoreOp>()
    }

    pub fn set_stencil_store_op(&mut self, value: &GPUStoreOp) {
        self.inner.set("stencilStoreOp", value);
    }
}
impl GPURenderPassDepthStencilAttachment {
    pub fn stencil_read_only(&self) -> bool {
        self.inner.get("stencilReadOnly").as_::<bool>()
    }

    pub fn set_stencil_read_only(&mut self, value: bool) {
        self.inner.set("stencilReadOnly", value);
    }
}
