use super::*;

/// The XRWebGLLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLLayerInit {
    inner: Any,
}

impl FromVal for XRWebGLLayerInit {
    fn from_val(v: &Any) -> Self {
        XRWebGLLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRWebGLLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRWebGLLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRWebGLLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRWebGLLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRWebGLLayerInit> for Any {
    fn from(s: XRWebGLLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRWebGLLayerInit> for Any {
    fn from(s: &XRWebGLLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRWebGLLayerInit {
    /// Getter of the `antialias` attribute.
    pub fn antialias(&self) -> bool {
        self.inner.get("antialias").as_::<bool>()
    }

    /// Setter of the `antialias` attribute.
    pub fn set_antialias(&mut self, value: bool) {
        self.inner.set("antialias", value);
    }
}
impl XRWebGLLayerInit {
    /// Getter of the `depth` attribute.
    pub fn depth(&self) -> bool {
        self.inner.get("depth").as_::<bool>()
    }

    /// Setter of the `depth` attribute.
    pub fn set_depth(&mut self, value: bool) {
        self.inner.set("depth", value);
    }
}
impl XRWebGLLayerInit {
    /// Getter of the `stencil` attribute.
    pub fn stencil(&self) -> bool {
        self.inner.get("stencil").as_::<bool>()
    }

    /// Setter of the `stencil` attribute.
    pub fn set_stencil(&mut self, value: bool) {
        self.inner.set("stencil", value);
    }
}
impl XRWebGLLayerInit {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
impl XRWebGLLayerInit {
    /// Getter of the `ignoreDepthValues` attribute.
    pub fn ignore_depth_values(&self) -> bool {
        self.inner.get("ignoreDepthValues").as_::<bool>()
    }

    /// Setter of the `ignoreDepthValues` attribute.
    pub fn set_ignore_depth_values(&mut self, value: bool) {
        self.inner.set("ignoreDepthValues", value);
    }
}
impl XRWebGLLayerInit {
    /// Getter of the `framebufferScaleFactor` attribute.
    pub fn framebuffer_scale_factor(&self) -> f64 {
        self.inner.get("framebufferScaleFactor").as_::<f64>()
    }

    /// Setter of the `framebufferScaleFactor` attribute.
    pub fn set_framebuffer_scale_factor(&mut self, value: f64) {
        self.inner.set("framebufferScaleFactor", value);
    }
}
