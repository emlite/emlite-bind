use super::*;

#[derive(Clone, Debug)]
pub struct XRWebGLLayer {
    inner: XRLayer,
}
impl FromVal for XRWebGLLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRWebGLLayer {
            inner: XRLayer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRWebGLLayer {
    type Target = XRLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRWebGLLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRWebGLLayer> for emlite::Val {
    fn from(s: XRWebGLLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRWebGLLayer {
    pub fn new0(session: XRSession, context: jsbind::Any) -> XRWebGLLayer {
        Self {
            inner: emlite::Val::global("XRWebGLLayer")
                .new(&[session.into(), context.into()])
                .as_::<XRLayer>(),
        }
    }

    pub fn new1(session: XRSession, context: jsbind::Any, layer_init: jsbind::Any) -> XRWebGLLayer {
        Self {
            inner: emlite::Val::global("XRWebGLLayer")
                .new(&[session.into(), context.into(), layer_init.into()])
                .as_::<XRLayer>(),
        }
    }
}
impl XRWebGLLayer {
    pub fn antialias(&self) -> bool {
        self.inner.get("antialias").as_::<bool>()
    }
}
impl XRWebGLLayer {
    pub fn ignore_depth_values(&self) -> bool {
        self.inner.get("ignoreDepthValues").as_::<bool>()
    }
}
impl XRWebGLLayer {
    pub fn fixed_foveation(&self) -> f32 {
        self.inner.get("fixedFoveation").as_::<f32>()
    }

    pub fn set_fixed_foveation(&mut self, value: f32) {
        self.inner.set("fixedFoveation", value);
    }
}
impl XRWebGLLayer {
    pub fn framebuffer(&self) -> WebGLFramebuffer {
        self.inner.get("framebuffer").as_::<WebGLFramebuffer>()
    }
}
impl XRWebGLLayer {
    pub fn framebuffer_width(&self) -> u32 {
        self.inner.get("framebufferWidth").as_::<u32>()
    }
}
impl XRWebGLLayer {
    pub fn framebuffer_height(&self) -> u32 {
        self.inner.get("framebufferHeight").as_::<u32>()
    }
}
impl XRWebGLLayer {
    pub fn get_viewport(&self, view: XRView) -> XRViewport {
        self.inner
            .call("getViewport", &[view.into()])
            .as_::<XRViewport>()
    }
}
impl XRWebGLLayer {
    pub fn get_native_framebuffer_scale_factor(session: XRSession) -> f64 {
        emlite::Val::global("xrwebgllayer")
            .call("getNativeFramebufferScaleFactor", &[session.into()])
            .as_::<f64>()
    }
}
