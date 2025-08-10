use super::*;

/// The PointerEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerEventInit {
    inner: Any,
}

impl FromVal for PointerEventInit {
    fn from_val(v: &Any) -> Self {
        PointerEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PointerEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PointerEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PointerEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PointerEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PointerEventInit> for Any {
    fn from(s: PointerEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PointerEventInit> for Any {
    fn from(s: &PointerEventInit) -> Any {
        s.inner.clone()
    }
}

impl PointerEventInit {
    /// Getter of the `pointerId` attribute.
    pub fn pointer_id(&self) -> i32 {
        self.inner.get("pointerId").as_::<i32>()
    }

    /// Setter of the `pointerId` attribute.
    pub fn set_pointer_id(&mut self, value: i32) {
        self.inner.set("pointerId", value);
    }
}
impl PointerEventInit {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl PointerEventInit {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: f64) {
        self.inner.set("height", value);
    }
}
impl PointerEventInit {
    /// Getter of the `pressure` attribute.
    pub fn pressure(&self) -> f32 {
        self.inner.get("pressure").as_::<f32>()
    }

    /// Setter of the `pressure` attribute.
    pub fn set_pressure(&mut self, value: f32) {
        self.inner.set("pressure", value);
    }
}
impl PointerEventInit {
    /// Getter of the `tangentialPressure` attribute.
    pub fn tangential_pressure(&self) -> f32 {
        self.inner.get("tangentialPressure").as_::<f32>()
    }

    /// Setter of the `tangentialPressure` attribute.
    pub fn set_tangential_pressure(&mut self, value: f32) {
        self.inner.set("tangentialPressure", value);
    }
}
impl PointerEventInit {
    /// Getter of the `tiltX` attribute.
    pub fn tilt_x(&self) -> i32 {
        self.inner.get("tiltX").as_::<i32>()
    }

    /// Setter of the `tiltX` attribute.
    pub fn set_tilt_x(&mut self, value: i32) {
        self.inner.set("tiltX", value);
    }
}
impl PointerEventInit {
    /// Getter of the `tiltY` attribute.
    pub fn tilt_y(&self) -> i32 {
        self.inner.get("tiltY").as_::<i32>()
    }

    /// Setter of the `tiltY` attribute.
    pub fn set_tilt_y(&mut self, value: i32) {
        self.inner.set("tiltY", value);
    }
}
impl PointerEventInit {
    /// Getter of the `twist` attribute.
    pub fn twist(&self) -> i32 {
        self.inner.get("twist").as_::<i32>()
    }

    /// Setter of the `twist` attribute.
    pub fn set_twist(&mut self, value: i32) {
        self.inner.set("twist", value);
    }
}
impl PointerEventInit {
    /// Getter of the `altitudeAngle` attribute.
    pub fn altitude_angle(&self) -> f64 {
        self.inner.get("altitudeAngle").as_::<f64>()
    }

    /// Setter of the `altitudeAngle` attribute.
    pub fn set_altitude_angle(&mut self, value: f64) {
        self.inner.set("altitudeAngle", value);
    }
}
impl PointerEventInit {
    /// Getter of the `azimuthAngle` attribute.
    pub fn azimuth_angle(&self) -> f64 {
        self.inner.get("azimuthAngle").as_::<f64>()
    }

    /// Setter of the `azimuthAngle` attribute.
    pub fn set_azimuth_angle(&mut self, value: f64) {
        self.inner.set("azimuthAngle", value);
    }
}
impl PointerEventInit {
    /// Getter of the `pointerType` attribute.
    pub fn pointer_type(&self) -> JsString {
        self.inner.get("pointerType").as_::<JsString>()
    }

    /// Setter of the `pointerType` attribute.
    pub fn set_pointer_type(&mut self, value: &JsString) {
        self.inner.set("pointerType", value);
    }
}
impl PointerEventInit {
    /// Getter of the `isPrimary` attribute.
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }

    /// Setter of the `isPrimary` attribute.
    pub fn set_is_primary(&mut self, value: bool) {
        self.inner.set("isPrimary", value);
    }
}
impl PointerEventInit {
    /// Getter of the `persistentDeviceId` attribute.
    pub fn persistent_device_id(&self) -> i32 {
        self.inner.get("persistentDeviceId").as_::<i32>()
    }

    /// Setter of the `persistentDeviceId` attribute.
    pub fn set_persistent_device_id(&mut self, value: i32) {
        self.inner.set("persistentDeviceId", value);
    }
}
impl PointerEventInit {
    /// Getter of the `coalescedEvents` attribute.
    pub fn coalesced_events(&self) -> TypedArray<PointerEvent> {
        self.inner
            .get("coalescedEvents")
            .as_::<TypedArray<PointerEvent>>()
    }

    /// Setter of the `coalescedEvents` attribute.
    pub fn set_coalesced_events(&mut self, value: &TypedArray<PointerEvent>) {
        self.inner.set("coalescedEvents", value);
    }
}
impl PointerEventInit {
    /// Getter of the `predictedEvents` attribute.
    pub fn predicted_events(&self) -> TypedArray<PointerEvent> {
        self.inner
            .get("predictedEvents")
            .as_::<TypedArray<PointerEvent>>()
    }

    /// Setter of the `predictedEvents` attribute.
    pub fn set_predicted_events(&mut self, value: &TypedArray<PointerEvent>) {
        self.inner.set("predictedEvents", value);
    }
}
