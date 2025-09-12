use super::*;

/// The XRWebGLLayer class.
/// [`XRWebGLLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLLayer {
    inner: XRLayer,
}

impl FromVal for XRWebGLLayer {
    fn from_val(v: &Any) -> Self {
        XRWebGLLayer {
            inner: XRLayer::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRWebGLLayer {
    type Target = XRLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRWebGLLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRWebGLLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRWebGLLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRWebGLLayer> for Any {
    fn from(s: XRWebGLLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRWebGLLayer> for Any {
    fn from(s: &XRWebGLLayer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRWebGLLayer);

impl XRWebGLLayer {
    /// Getter of the `antialias` attribute.
    /// [`XRWebGLLayer.antialias`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/antialias)
    pub fn antialias(&self) -> bool {
        self.inner.get("antialias").as_::<bool>()
    }
}
impl XRWebGLLayer {
    /// Getter of the `ignoreDepthValues` attribute.
    /// [`XRWebGLLayer.ignoreDepthValues`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/ignoreDepthValues)
    pub fn ignore_depth_values(&self) -> bool {
        self.inner.get("ignoreDepthValues").as_::<bool>()
    }
}
impl XRWebGLLayer {
    /// Getter of the `fixedFoveation` attribute.
    /// [`XRWebGLLayer.fixedFoveation`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)
    pub fn fixed_foveation(&self) -> f32 {
        self.inner.get("fixedFoveation").as_::<f32>()
    }

    /// Setter of the `fixedFoveation` attribute.
    /// [`XRWebGLLayer.fixedFoveation`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)
    pub fn set_fixed_foveation(&mut self, value: f32) {
        self.inner.set("fixedFoveation", value);
    }
}
impl XRWebGLLayer {
    /// Getter of the `framebuffer` attribute.
    /// [`XRWebGLLayer.framebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebuffer)
    pub fn framebuffer(&self) -> WebGLFramebuffer {
        self.inner.get("framebuffer").as_::<WebGLFramebuffer>()
    }
}
impl XRWebGLLayer {
    /// Getter of the `framebufferWidth` attribute.
    /// [`XRWebGLLayer.framebufferWidth`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferWidth)
    pub fn framebuffer_width(&self) -> u32 {
        self.inner.get("framebufferWidth").as_::<u32>()
    }
}
impl XRWebGLLayer {
    /// Getter of the `framebufferHeight` attribute.
    /// [`XRWebGLLayer.framebufferHeight`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferHeight)
    pub fn framebuffer_height(&self) -> u32 {
        self.inner.get("framebufferHeight").as_::<u32>()
    }
}

impl XRWebGLLayer {
    /// The `new XRWebGLLayer(..)` constructor, creating a new XRWebGLLayer instance
    pub fn new0(session: &XRSession, context: &Any) -> XRWebGLLayer {
        Self {
            inner: Any::global("XRWebGLLayer")
                .new(&[session.into(), context.into()])
                .as_::<XRLayer>(),
        }
    }

    /// The `new XRWebGLLayer(..)` constructor, creating a new XRWebGLLayer instance
    pub fn new1(session: &XRSession, context: &Any, layer_init: &XRWebGLLayerInit) -> XRWebGLLayer {
        Self {
            inner: Any::global("XRWebGLLayer")
                .new(&[session.into(), context.into(), layer_init.into()])
                .as_::<XRLayer>(),
        }
    }
}
impl XRWebGLLayer {
    /// The getViewport method.
    /// [`XRWebGLLayer.getViewport`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getViewport)
    pub fn get_viewport(&self, view: &XRView) -> XRViewport {
        self.inner
            .call("getViewport", &[view.into()])
            .as_::<XRViewport>()
    }
}
impl XRWebGLLayer {
    /// The getNativeFramebufferScaleFactor method.
    /// [`XRWebGLLayer.getNativeFramebufferScaleFactor`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getNativeFramebufferScaleFactor)
    pub fn get_native_framebuffer_scale_factor(session: &XRSession) -> f64 {
        Any::global("XRWebGLLayer")
            .call("getNativeFramebufferScaleFactor", &[session.into()])
            .as_::<f64>()
    }
}
