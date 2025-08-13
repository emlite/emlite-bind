use super::*;




/// The GPUDepthStencilState dictionary.
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
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `depthWriteEnabled` attribute.
    pub fn depth_write_enabled(&self) -> bool {
        self.inner.get("depthWriteEnabled").as_::<bool>()
    }

    /// Setter of the `depthWriteEnabled` attribute.
    pub fn set_depth_write_enabled(&mut self, value: bool) {
        self.inner.set("depthWriteEnabled", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `depthCompare` attribute.
    pub fn depth_compare(&self) -> GPUCompareFunction {
        self.inner.get("depthCompare").as_::<GPUCompareFunction>()
    }

    /// Setter of the `depthCompare` attribute.
    pub fn set_depth_compare(&mut self, value: &GPUCompareFunction) {
        self.inner.set("depthCompare", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `stencilFront` attribute.
    pub fn stencil_front(&self) -> GPUStencilFaceState {
        self.inner.get("stencilFront").as_::<GPUStencilFaceState>()
    }

    /// Setter of the `stencilFront` attribute.
    pub fn set_stencil_front(&mut self, value: &GPUStencilFaceState) {
        self.inner.set("stencilFront", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `stencilBack` attribute.
    pub fn stencil_back(&self) -> GPUStencilFaceState {
        self.inner.get("stencilBack").as_::<GPUStencilFaceState>()
    }

    /// Setter of the `stencilBack` attribute.
    pub fn set_stencil_back(&mut self, value: &GPUStencilFaceState) {
        self.inner.set("stencilBack", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `stencilReadMask` attribute.
    pub fn stencil_read_mask(&self) -> Any {
        self.inner.get("stencilReadMask").as_::<Any>()
    }

    /// Setter of the `stencilReadMask` attribute.
    pub fn set_stencil_read_mask(&mut self, value: &Any) {
        self.inner.set("stencilReadMask", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `stencilWriteMask` attribute.
    pub fn stencil_write_mask(&self) -> Any {
        self.inner.get("stencilWriteMask").as_::<Any>()
    }

    /// Setter of the `stencilWriteMask` attribute.
    pub fn set_stencil_write_mask(&mut self, value: &Any) {
        self.inner.set("stencilWriteMask", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `depthBias` attribute.
    pub fn depth_bias(&self) -> Any {
        self.inner.get("depthBias").as_::<Any>()
    }

    /// Setter of the `depthBias` attribute.
    pub fn set_depth_bias(&mut self, value: &Any) {
        self.inner.set("depthBias", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `depthBiasSlopeScale` attribute.
    pub fn depth_bias_slope_scale(&self) -> f32 {
        self.inner.get("depthBiasSlopeScale").as_::<f32>()
    }

    /// Setter of the `depthBiasSlopeScale` attribute.
    pub fn set_depth_bias_slope_scale(&mut self, value: f32) {
        self.inner.set("depthBiasSlopeScale", value);
    }
}
impl GPUDepthStencilState {
    /// Getter of the `depthBiasClamp` attribute.
    pub fn depth_bias_clamp(&self) -> f32 {
        self.inner.get("depthBiasClamp").as_::<f32>()
    }

    /// Setter of the `depthBiasClamp` attribute.
    pub fn set_depth_bias_clamp(&mut self, value: f32) {
        self.inner.set("depthBiasClamp", value);
    }
}
