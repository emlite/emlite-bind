use super::*;




/// The XRQuadLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRQuadLayerInit {
    inner: Any,
}

impl FromVal for XRQuadLayerInit {
    fn from_val(v: &Any) -> Self {
        XRQuadLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRQuadLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRQuadLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRQuadLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRQuadLayerInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRQuadLayerInit> for Any {
    fn from(s: XRQuadLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRQuadLayerInit> for Any {
    fn from(s: &XRQuadLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRQuadLayerInit {
    /// Getter of the `textureType` attribute.
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    /// Setter of the `textureType` attribute.
    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRQuadLayerInit {
    /// Getter of the `transform` attribute.
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRQuadLayerInit {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> f32 {
        self.inner.get("width").as_::<f32>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: f32) {
        self.inner.set("width", value);
    }
}
impl XRQuadLayerInit {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> f32 {
        self.inner.get("height").as_::<f32>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: f32) {
        self.inner.set("height", value);
    }
}
