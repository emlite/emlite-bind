use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDepthStencilState {
    inner: Any,
}
impl FromVal for GPUDepthStencilState {
    fn from_val(v: &Any) -> Self {
        GPUDepthStencilState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUDepthStencilState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUDepthStencilState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUDepthStencilState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUDepthStencilState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUDepthStencilState> for Any {
    fn from(s: GPUDepthStencilState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUDepthStencilState> for Any {
    fn from(s: &GPUDepthStencilState) -> Any {
        s.inner.clone()
    }
}

impl GPUDepthStencilState {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUDepthStencilState {
    pub fn depth_write_enabled(&self) -> bool {
        self.inner.get("depthWriteEnabled").as_::<bool>()
    }

    pub fn set_depth_write_enabled(&mut self, value: bool) {
        self.inner.set("depthWriteEnabled", value);
    }
}
impl GPUDepthStencilState {
    pub fn depth_compare(&self) -> GPUCompareFunction {
        self.inner.get("depthCompare").as_::<GPUCompareFunction>()
    }

    pub fn set_depth_compare(&mut self, value: &GPUCompareFunction) {
        self.inner.set("depthCompare", value);
    }
}
impl GPUDepthStencilState {
    pub fn stencil_front(&self) -> GPUStencilFaceState {
        self.inner.get("stencilFront").as_::<GPUStencilFaceState>()
    }

    pub fn set_stencil_front(&mut self, value: &GPUStencilFaceState) {
        self.inner.set("stencilFront", value);
    }
}
impl GPUDepthStencilState {
    pub fn stencil_back(&self) -> GPUStencilFaceState {
        self.inner.get("stencilBack").as_::<GPUStencilFaceState>()
    }

    pub fn set_stencil_back(&mut self, value: &GPUStencilFaceState) {
        self.inner.set("stencilBack", value);
    }
}
impl GPUDepthStencilState {
    pub fn stencil_read_mask(&self) -> Any {
        self.inner.get("stencilReadMask").as_::<Any>()
    }

    pub fn set_stencil_read_mask(&mut self, value: &Any) {
        self.inner.set("stencilReadMask", value);
    }
}
impl GPUDepthStencilState {
    pub fn stencil_write_mask(&self) -> Any {
        self.inner.get("stencilWriteMask").as_::<Any>()
    }

    pub fn set_stencil_write_mask(&mut self, value: &Any) {
        self.inner.set("stencilWriteMask", value);
    }
}
impl GPUDepthStencilState {
    pub fn depth_bias(&self) -> Any {
        self.inner.get("depthBias").as_::<Any>()
    }

    pub fn set_depth_bias(&mut self, value: &Any) {
        self.inner.set("depthBias", value);
    }
}
impl GPUDepthStencilState {
    pub fn depth_bias_slope_scale(&self) -> f32 {
        self.inner.get("depthBiasSlopeScale").as_::<f32>()
    }

    pub fn set_depth_bias_slope_scale(&mut self, value: f32) {
        self.inner.set("depthBiasSlopeScale", value);
    }
}
impl GPUDepthStencilState {
    pub fn depth_bias_clamp(&self) -> f32 {
        self.inner.get("depthBiasClamp").as_::<f32>()
    }

    pub fn set_depth_bias_clamp(&mut self, value: f32) {
        self.inner.set("depthBiasClamp", value);
    }
}
