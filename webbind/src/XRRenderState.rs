use super::*;

#[derive(Clone, Debug)]
pub struct XRRenderState {
    inner: emlite::Val,
}
impl FromVal for XRRenderState {
    fn from_val(v: &emlite::Val) -> Self {
        XRRenderState {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRRenderState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRRenderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRRenderState> for emlite::Val {
    fn from(s: XRRenderState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRRenderState {
    pub fn depth_near(&self) -> f64 {
        self.inner.get("depthNear").as_::<f64>()
    }
}
impl XRRenderState {
    pub fn depth_far(&self) -> f64 {
        self.inner.get("depthFar").as_::<f64>()
    }
}
impl XRRenderState {
    pub fn passthrough_fully_obscured(&self) -> bool {
        self.inner.get("passthroughFullyObscured").as_::<bool>()
    }
}
impl XRRenderState {
    pub fn inline_vertical_field_of_view(&self) -> f64 {
        self.inner.get("inlineVerticalFieldOfView").as_::<f64>()
    }
}
impl XRRenderState {
    pub fn base_layer(&self) -> XRWebGLLayer {
        self.inner.get("baseLayer").as_::<XRWebGLLayer>()
    }
}
impl XRRenderState {
    pub fn layers(&self) -> jsbind::FrozenArray<XRLayer> {
        self.inner
            .get("layers")
            .as_::<jsbind::FrozenArray<XRLayer>>()
    }
}
