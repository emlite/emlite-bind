use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerEvent {
    inner: MouseEvent,
}
impl FromVal for PointerEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PointerEvent {
            inner: MouseEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PointerEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PointerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PointerEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PointerEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PointerEvent> for emlite::Val {
    fn from(s: PointerEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PointerEvent);

impl PointerEvent {
    pub fn new0(type_: DOMString) -> PointerEvent {
        Self {
            inner: emlite::Val::global("PointerEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PointerEvent {
        Self {
            inner: emlite::Val::global("PointerEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<MouseEvent>(),
        }
    }
}
impl PointerEvent {
    pub fn pointer_id(&self) -> i32 {
        self.inner.get("pointerId").as_::<i32>()
    }
}
impl PointerEvent {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl PointerEvent {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl PointerEvent {
    pub fn pressure(&self) -> f32 {
        self.inner.get("pressure").as_::<f32>()
    }
}
impl PointerEvent {
    pub fn tangential_pressure(&self) -> f32 {
        self.inner.get("tangentialPressure").as_::<f32>()
    }
}
impl PointerEvent {
    pub fn tilt_x(&self) -> i32 {
        self.inner.get("tiltX").as_::<i32>()
    }
}
impl PointerEvent {
    pub fn tilt_y(&self) -> i32 {
        self.inner.get("tiltY").as_::<i32>()
    }
}
impl PointerEvent {
    pub fn twist(&self) -> i32 {
        self.inner.get("twist").as_::<i32>()
    }
}
impl PointerEvent {
    pub fn altitude_angle(&self) -> f64 {
        self.inner.get("altitudeAngle").as_::<f64>()
    }
}
impl PointerEvent {
    pub fn azimuth_angle(&self) -> f64 {
        self.inner.get("azimuthAngle").as_::<f64>()
    }
}
impl PointerEvent {
    pub fn pointer_type(&self) -> DOMString {
        self.inner.get("pointerType").as_::<DOMString>()
    }
}
impl PointerEvent {
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl PointerEvent {
    pub fn persistent_device_id(&self) -> i32 {
        self.inner.get("persistentDeviceId").as_::<i32>()
    }
}
impl PointerEvent {
    pub fn get_coalesced_events(&self) -> Sequence<PointerEvent> {
        self.inner
            .call("getCoalescedEvents", &[])
            .as_::<Sequence<PointerEvent>>()
    }
}
impl PointerEvent {
    pub fn get_predicted_events(&self) -> Sequence<PointerEvent> {
        self.inner
            .call("getPredictedEvents", &[])
            .as_::<Sequence<PointerEvent>>()
    }
}
