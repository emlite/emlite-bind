use super::*;




/// The XRMediaEquirectLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaEquirectLayerInit {
    inner: Any,
}

impl FromVal for XRMediaEquirectLayerInit {
    fn from_val(v: &Any) -> Self {
        XRMediaEquirectLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRMediaEquirectLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRMediaEquirectLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRMediaEquirectLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRMediaEquirectLayerInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRMediaEquirectLayerInit> for Any {
    fn from(s: XRMediaEquirectLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRMediaEquirectLayerInit> for Any {
    fn from(s: &XRMediaEquirectLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRMediaEquirectLayerInit {
    /// Getter of the `transform` attribute.
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRMediaEquirectLayerInit {
    /// Getter of the `radius` attribute.
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    /// Setter of the `radius` attribute.
    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRMediaEquirectLayerInit {
    /// Getter of the `centralHorizontalAngle` attribute.
    pub fn central_horizontal_angle(&self) -> f32 {
        self.inner.get("centralHorizontalAngle").as_::<f32>()
    }

    /// Setter of the `centralHorizontalAngle` attribute.
    pub fn set_central_horizontal_angle(&mut self, value: f32) {
        self.inner.set("centralHorizontalAngle", value);
    }
}
impl XRMediaEquirectLayerInit {
    /// Getter of the `upperVerticalAngle` attribute.
    pub fn upper_vertical_angle(&self) -> f32 {
        self.inner.get("upperVerticalAngle").as_::<f32>()
    }

    /// Setter of the `upperVerticalAngle` attribute.
    pub fn set_upper_vertical_angle(&mut self, value: f32) {
        self.inner.set("upperVerticalAngle", value);
    }
}
impl XRMediaEquirectLayerInit {
    /// Getter of the `lowerVerticalAngle` attribute.
    pub fn lower_vertical_angle(&self) -> f32 {
        self.inner.get("lowerVerticalAngle").as_::<f32>()
    }

    /// Setter of the `lowerVerticalAngle` attribute.
    pub fn set_lower_vertical_angle(&mut self, value: f32) {
        self.inner.set("lowerVerticalAngle", value);
    }
}
