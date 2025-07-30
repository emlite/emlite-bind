use super::*;

/// The XRRenderState class.
/// [`XRRenderState`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRenderState {
    inner: Any,
}
impl FromVal for XRRenderState {
    fn from_val(v: &Any) -> Self {
        XRRenderState {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRRenderState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRenderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRRenderState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRRenderState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRRenderState> for Any {
    fn from(s: XRRenderState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRRenderState> for Any {
    fn from(s: &XRRenderState) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRRenderState);

impl XRRenderState {
    /// Getter of the `depthNear` attribute.
    /// [`XRRenderState.depthNear`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/depthNear)
    pub fn depth_near(&self) -> f64 {
        self.inner.get("depthNear").as_::<f64>()
    }
}
impl XRRenderState {
    /// Getter of the `depthFar` attribute.
    /// [`XRRenderState.depthFar`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/depthFar)
    pub fn depth_far(&self) -> f64 {
        self.inner.get("depthFar").as_::<f64>()
    }
}
impl XRRenderState {
    /// Getter of the `passthroughFullyObscured` attribute.
    /// [`XRRenderState.passthroughFullyObscured`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/passthroughFullyObscured)
    pub fn passthrough_fully_obscured(&self) -> bool {
        self.inner.get("passthroughFullyObscured").as_::<bool>()
    }
}
impl XRRenderState {
    /// Getter of the `inlineVerticalFieldOfView` attribute.
    /// [`XRRenderState.inlineVerticalFieldOfView`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/inlineVerticalFieldOfView)
    pub fn inline_vertical_field_of_view(&self) -> f64 {
        self.inner.get("inlineVerticalFieldOfView").as_::<f64>()
    }
}
impl XRRenderState {
    /// Getter of the `baseLayer` attribute.
    /// [`XRRenderState.baseLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/baseLayer)
    pub fn base_layer(&self) -> XRWebGLLayer {
        self.inner.get("baseLayer").as_::<XRWebGLLayer>()
    }
}
impl XRRenderState {
    /// Getter of the `layers` attribute.
    /// [`XRRenderState.layers`](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/layers)
    pub fn layers(&self) -> TypedArray<XRLayer> {
        self.inner.get("layers").as_::<TypedArray<XRLayer>>()
    }
}
