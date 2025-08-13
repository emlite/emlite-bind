use super::*;




/// The XRRenderStateInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRenderStateInit {
    inner: Any,
}

impl FromVal for XRRenderStateInit {
    fn from_val(v: &Any) -> Self {
        XRRenderStateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRRenderStateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRRenderStateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRRenderStateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRRenderStateInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRRenderStateInit> for Any {
    fn from(s: XRRenderStateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRRenderStateInit> for Any {
    fn from(s: &XRRenderStateInit) -> Any {
        s.inner.clone()
    }
}

impl XRRenderStateInit {
    /// Getter of the `depthNear` attribute.
    pub fn depth_near(&self) -> f64 {
        self.inner.get("depthNear").as_::<f64>()
    }

    /// Setter of the `depthNear` attribute.
    pub fn set_depth_near(&mut self, value: f64) {
        self.inner.set("depthNear", value);
    }
}
impl XRRenderStateInit {
    /// Getter of the `depthFar` attribute.
    pub fn depth_far(&self) -> f64 {
        self.inner.get("depthFar").as_::<f64>()
    }

    /// Setter of the `depthFar` attribute.
    pub fn set_depth_far(&mut self, value: f64) {
        self.inner.set("depthFar", value);
    }
}
impl XRRenderStateInit {
    /// Getter of the `passthroughFullyObscured` attribute.
    pub fn passthrough_fully_obscured(&self) -> bool {
        self.inner.get("passthroughFullyObscured").as_::<bool>()
    }

    /// Setter of the `passthroughFullyObscured` attribute.
    pub fn set_passthrough_fully_obscured(&mut self, value: bool) {
        self.inner.set("passthroughFullyObscured", value);
    }
}
impl XRRenderStateInit {
    /// Getter of the `inlineVerticalFieldOfView` attribute.
    pub fn inline_vertical_field_of_view(&self) -> f64 {
        self.inner.get("inlineVerticalFieldOfView").as_::<f64>()
    }

    /// Setter of the `inlineVerticalFieldOfView` attribute.
    pub fn set_inline_vertical_field_of_view(&mut self, value: f64) {
        self.inner.set("inlineVerticalFieldOfView", value);
    }
}
impl XRRenderStateInit {
    /// Getter of the `baseLayer` attribute.
    pub fn base_layer(&self) -> XRWebGLLayer {
        self.inner.get("baseLayer").as_::<XRWebGLLayer>()
    }

    /// Setter of the `baseLayer` attribute.
    pub fn set_base_layer(&mut self, value: &XRWebGLLayer) {
        self.inner.set("baseLayer", value);
    }
}
impl XRRenderStateInit {
    /// Getter of the `layers` attribute.
    pub fn layers(&self) -> TypedArray<XRLayer> {
        self.inner.get("layers").as_::<TypedArray<XRLayer>>()
    }

    /// Setter of the `layers` attribute.
    pub fn set_layers(&mut self, value: &TypedArray<XRLayer>) {
        self.inner.set("layers", value);
    }
}
