use super::*;




/// The XRCylinderLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCylinderLayerInit {
    inner: Any,
}

impl FromVal for XRCylinderLayerInit {
    fn from_val(v: &Any) -> Self {
        XRCylinderLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRCylinderLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRCylinderLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRCylinderLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRCylinderLayerInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRCylinderLayerInit> for Any {
    fn from(s: XRCylinderLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRCylinderLayerInit> for Any {
    fn from(s: &XRCylinderLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRCylinderLayerInit {
    /// Getter of the `textureType` attribute.
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    /// Setter of the `textureType` attribute.
    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRCylinderLayerInit {
    /// Getter of the `transform` attribute.
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRCylinderLayerInit {
    /// Getter of the `radius` attribute.
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    /// Setter of the `radius` attribute.
    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRCylinderLayerInit {
    /// Getter of the `centralAngle` attribute.
    pub fn central_angle(&self) -> f32 {
        self.inner.get("centralAngle").as_::<f32>()
    }

    /// Setter of the `centralAngle` attribute.
    pub fn set_central_angle(&mut self, value: f32) {
        self.inner.set("centralAngle", value);
    }
}
impl XRCylinderLayerInit {
    /// Getter of the `aspectRatio` attribute.
    pub fn aspect_ratio(&self) -> f32 {
        self.inner.get("aspectRatio").as_::<f32>()
    }

    /// Setter of the `aspectRatio` attribute.
    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.inner.set("aspectRatio", value);
    }
}
