use super::*;




/// The TouchInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TouchInit {
    inner: Any,
}

impl FromVal for TouchInit {
    fn from_val(v: &Any) -> Self {
        TouchInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TouchInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TouchInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TouchInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TouchInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TouchInit> for Any {
    fn from(s: TouchInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TouchInit> for Any {
    fn from(s: &TouchInit) -> Any {
        s.inner.clone()
    }
}

impl TouchInit {
    /// Getter of the `identifier` attribute.
    pub fn identifier(&self) -> i32 {
        self.inner.get("identifier").as_::<i32>()
    }

    /// Setter of the `identifier` attribute.
    pub fn set_identifier(&mut self, value: i32) {
        self.inner.set("identifier", value);
    }
}
impl TouchInit {
    /// Getter of the `target` attribute.
    pub fn target(&self) -> EventTarget {
        self.inner.get("target").as_::<EventTarget>()
    }

    /// Setter of the `target` attribute.
    pub fn set_target(&mut self, value: &EventTarget) {
        self.inner.set("target", value);
    }
}
impl TouchInit {
    /// Getter of the `clientX` attribute.
    pub fn client_x(&self) -> f64 {
        self.inner.get("clientX").as_::<f64>()
    }

    /// Setter of the `clientX` attribute.
    pub fn set_client_x(&mut self, value: f64) {
        self.inner.set("clientX", value);
    }
}
impl TouchInit {
    /// Getter of the `clientY` attribute.
    pub fn client_y(&self) -> f64 {
        self.inner.get("clientY").as_::<f64>()
    }

    /// Setter of the `clientY` attribute.
    pub fn set_client_y(&mut self, value: f64) {
        self.inner.set("clientY", value);
    }
}
impl TouchInit {
    /// Getter of the `screenX` attribute.
    pub fn screen_x(&self) -> f64 {
        self.inner.get("screenX").as_::<f64>()
    }

    /// Setter of the `screenX` attribute.
    pub fn set_screen_x(&mut self, value: f64) {
        self.inner.set("screenX", value);
    }
}
impl TouchInit {
    /// Getter of the `screenY` attribute.
    pub fn screen_y(&self) -> f64 {
        self.inner.get("screenY").as_::<f64>()
    }

    /// Setter of the `screenY` attribute.
    pub fn set_screen_y(&mut self, value: f64) {
        self.inner.set("screenY", value);
    }
}
impl TouchInit {
    /// Getter of the `pageX` attribute.
    pub fn page_x(&self) -> f64 {
        self.inner.get("pageX").as_::<f64>()
    }

    /// Setter of the `pageX` attribute.
    pub fn set_page_x(&mut self, value: f64) {
        self.inner.set("pageX", value);
    }
}
impl TouchInit {
    /// Getter of the `pageY` attribute.
    pub fn page_y(&self) -> f64 {
        self.inner.get("pageY").as_::<f64>()
    }

    /// Setter of the `pageY` attribute.
    pub fn set_page_y(&mut self, value: f64) {
        self.inner.set("pageY", value);
    }
}
impl TouchInit {
    /// Getter of the `radiusX` attribute.
    pub fn radius_x(&self) -> f32 {
        self.inner.get("radiusX").as_::<f32>()
    }

    /// Setter of the `radiusX` attribute.
    pub fn set_radius_x(&mut self, value: f32) {
        self.inner.set("radiusX", value);
    }
}
impl TouchInit {
    /// Getter of the `radiusY` attribute.
    pub fn radius_y(&self) -> f32 {
        self.inner.get("radiusY").as_::<f32>()
    }

    /// Setter of the `radiusY` attribute.
    pub fn set_radius_y(&mut self, value: f32) {
        self.inner.set("radiusY", value);
    }
}
impl TouchInit {
    /// Getter of the `rotationAngle` attribute.
    pub fn rotation_angle(&self) -> f32 {
        self.inner.get("rotationAngle").as_::<f32>()
    }

    /// Setter of the `rotationAngle` attribute.
    pub fn set_rotation_angle(&mut self, value: f32) {
        self.inner.set("rotationAngle", value);
    }
}
impl TouchInit {
    /// Getter of the `force` attribute.
    pub fn force(&self) -> f32 {
        self.inner.get("force").as_::<f32>()
    }

    /// Setter of the `force` attribute.
    pub fn set_force(&mut self, value: f32) {
        self.inner.set("force", value);
    }
}
impl TouchInit {
    /// Getter of the `altitudeAngle` attribute.
    pub fn altitude_angle(&self) -> f64 {
        self.inner.get("altitudeAngle").as_::<f64>()
    }

    /// Setter of the `altitudeAngle` attribute.
    pub fn set_altitude_angle(&mut self, value: f64) {
        self.inner.set("altitudeAngle", value);
    }
}
impl TouchInit {
    /// Getter of the `azimuthAngle` attribute.
    pub fn azimuth_angle(&self) -> f64 {
        self.inner.get("azimuthAngle").as_::<f64>()
    }

    /// Setter of the `azimuthAngle` attribute.
    pub fn set_azimuth_angle(&mut self, value: f64) {
        self.inner.set("azimuthAngle", value);
    }
}
impl TouchInit {
    /// Getter of the `touchType` attribute.
    pub fn touch_type(&self) -> TouchType {
        self.inner.get("touchType").as_::<TouchType>()
    }

    /// Setter of the `touchType` attribute.
    pub fn set_touch_type(&mut self, value: &TouchType) {
        self.inner.set("touchType", value);
    }
}
