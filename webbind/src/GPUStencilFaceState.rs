use super::*;




/// The GPUStencilFaceState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUStencilFaceState {
    inner: Any,
}

impl FromVal for GPUStencilFaceState {
    fn from_val(v: &Any) -> Self {
        GPUStencilFaceState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUStencilFaceState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUStencilFaceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUStencilFaceState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUStencilFaceState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUStencilFaceState> for Any {
    fn from(s: GPUStencilFaceState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUStencilFaceState> for Any {
    fn from(s: &GPUStencilFaceState) -> Any {
        s.inner.clone()
    }
}

impl GPUStencilFaceState {
    /// Getter of the `compare` attribute.
    pub fn compare(&self) -> GPUCompareFunction {
        self.inner.get("compare").as_::<GPUCompareFunction>()
    }

    /// Setter of the `compare` attribute.
    pub fn set_compare(&mut self, value: &GPUCompareFunction) {
        self.inner.set("compare", value);
    }
}
impl GPUStencilFaceState {
    /// Getter of the `failOp` attribute.
    pub fn fail_op(&self) -> GPUStencilOperation {
        self.inner.get("failOp").as_::<GPUStencilOperation>()
    }

    /// Setter of the `failOp` attribute.
    pub fn set_fail_op(&mut self, value: &GPUStencilOperation) {
        self.inner.set("failOp", value);
    }
}
impl GPUStencilFaceState {
    /// Getter of the `depthFailOp` attribute.
    pub fn depth_fail_op(&self) -> GPUStencilOperation {
        self.inner.get("depthFailOp").as_::<GPUStencilOperation>()
    }

    /// Setter of the `depthFailOp` attribute.
    pub fn set_depth_fail_op(&mut self, value: &GPUStencilOperation) {
        self.inner.set("depthFailOp", value);
    }
}
impl GPUStencilFaceState {
    /// Getter of the `passOp` attribute.
    pub fn pass_op(&self) -> GPUStencilOperation {
        self.inner.get("passOp").as_::<GPUStencilOperation>()
    }

    /// Setter of the `passOp` attribute.
    pub fn set_pass_op(&mut self, value: &GPUStencilOperation) {
        self.inner.set("passOp", value);
    }
}
