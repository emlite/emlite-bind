use super::*;

/// The XRProjectionLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRProjectionLayerInit {
    inner: Any,
}

impl FromVal for XRProjectionLayerInit {
    fn from_val(v: &Any) -> Self {
        XRProjectionLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRProjectionLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRProjectionLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRProjectionLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRProjectionLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRProjectionLayerInit> for Any {
    fn from(s: XRProjectionLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRProjectionLayerInit> for Any {
    fn from(s: &XRProjectionLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRProjectionLayerInit {
    /// Getter of the `textureType` attribute.
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    /// Setter of the `textureType` attribute.
    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRProjectionLayerInit {
    /// Getter of the `colorFormat` attribute.
    pub fn color_format(&self) -> Any {
        self.inner.get("colorFormat").as_::<Any>()
    }

    /// Setter of the `colorFormat` attribute.
    pub fn set_color_format(&mut self, value: &Any) {
        self.inner.set("colorFormat", value);
    }
}
impl XRProjectionLayerInit {
    /// Getter of the `depthFormat` attribute.
    pub fn depth_format(&self) -> Any {
        self.inner.get("depthFormat").as_::<Any>()
    }

    /// Setter of the `depthFormat` attribute.
    pub fn set_depth_format(&mut self, value: &Any) {
        self.inner.set("depthFormat", value);
    }
}
impl XRProjectionLayerInit {
    /// Getter of the `scaleFactor` attribute.
    pub fn scale_factor(&self) -> f64 {
        self.inner.get("scaleFactor").as_::<f64>()
    }

    /// Setter of the `scaleFactor` attribute.
    pub fn set_scale_factor(&mut self, value: f64) {
        self.inner.set("scaleFactor", value);
    }
}
impl XRProjectionLayerInit {
    /// Getter of the `clearOnAccess` attribute.
    pub fn clear_on_access(&self) -> bool {
        self.inner.get("clearOnAccess").as_::<bool>()
    }

    /// Setter of the `clearOnAccess` attribute.
    pub fn set_clear_on_access(&mut self, value: bool) {
        self.inner.set("clearOnAccess", value);
    }
}
