use super::*;

/// The GPURenderPassColorAttachment dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassColorAttachment {
    inner: Any,
}

impl FromVal for GPURenderPassColorAttachment {
    fn from_val(v: &Any) -> Self {
        GPURenderPassColorAttachment { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderPassColorAttachment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderPassColorAttachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderPassColorAttachment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderPassColorAttachment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderPassColorAttachment> for Any {
    fn from(s: GPURenderPassColorAttachment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderPassColorAttachment> for Any {
    fn from(s: &GPURenderPassColorAttachment) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPassColorAttachment {
    /// Getter of the `view` attribute.
    pub fn view(&self) -> GPUTextureView {
        self.inner.get("view").as_::<GPUTextureView>()
    }

    /// Setter of the `view` attribute.
    pub fn set_view(&mut self, value: &GPUTextureView) {
        self.inner.set("view", value);
    }
}
impl GPURenderPassColorAttachment {
    /// Getter of the `depthSlice` attribute.
    pub fn depth_slice(&self) -> Any {
        self.inner.get("depthSlice").as_::<Any>()
    }

    /// Setter of the `depthSlice` attribute.
    pub fn set_depth_slice(&mut self, value: &Any) {
        self.inner.set("depthSlice", value);
    }
}
impl GPURenderPassColorAttachment {
    /// Getter of the `resolveTarget` attribute.
    pub fn resolve_target(&self) -> GPUTextureView {
        self.inner.get("resolveTarget").as_::<GPUTextureView>()
    }

    /// Setter of the `resolveTarget` attribute.
    pub fn set_resolve_target(&mut self, value: &GPUTextureView) {
        self.inner.set("resolveTarget", value);
    }
}
impl GPURenderPassColorAttachment {
    /// Getter of the `clearValue` attribute.
    pub fn clear_value(&self) -> Any {
        self.inner.get("clearValue").as_::<Any>()
    }

    /// Setter of the `clearValue` attribute.
    pub fn set_clear_value(&mut self, value: &Any) {
        self.inner.set("clearValue", value);
    }
}
impl GPURenderPassColorAttachment {
    /// Getter of the `loadOp` attribute.
    pub fn load_op(&self) -> GPULoadOp {
        self.inner.get("loadOp").as_::<GPULoadOp>()
    }

    /// Setter of the `loadOp` attribute.
    pub fn set_load_op(&mut self, value: &GPULoadOp) {
        self.inner.set("loadOp", value);
    }
}
impl GPURenderPassColorAttachment {
    /// Getter of the `storeOp` attribute.
    pub fn store_op(&self) -> GPUStoreOp {
        self.inner.get("storeOp").as_::<GPUStoreOp>()
    }

    /// Setter of the `storeOp` attribute.
    pub fn set_store_op(&mut self, value: &GPUStoreOp) {
        self.inner.set("storeOp", value);
    }
}
