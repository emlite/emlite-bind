use super::*;

/// The PointerEvent class.
/// [`PointerEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerEvent {
    inner: MouseEvent,
}
impl FromVal for PointerEvent {
    fn from_val(v: &Any) -> Self {
        PointerEvent {
            inner: MouseEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for PointerEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PointerEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PointerEvent> for Any {
    fn from(s: PointerEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PointerEvent> for Any {
    fn from(s: &PointerEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PointerEvent);

impl PointerEvent {
    /// The `new PointerEvent(..)` constructor, creating a new PointerEvent instance
    pub fn new0(type_: &JsString) -> PointerEvent {
        Self {
            inner: Any::global("PointerEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    /// The `new PointerEvent(..)` constructor, creating a new PointerEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &PointerEventInit) -> PointerEvent {
        Self {
            inner: Any::global("PointerEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<MouseEvent>(),
        }
    }
}
impl PointerEvent {
    /// Getter of the `pointerId` attribute.
    /// [`PointerEvent.pointerId`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)
    pub fn pointer_id(&self) -> i32 {
        self.inner.get("pointerId").as_::<i32>()
    }
}
impl PointerEvent {
    /// Getter of the `width` attribute.
    /// [`PointerEvent.width`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl PointerEvent {
    /// Getter of the `height` attribute.
    /// [`PointerEvent.height`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl PointerEvent {
    /// Getter of the `pressure` attribute.
    /// [`PointerEvent.pressure`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)
    pub fn pressure(&self) -> f32 {
        self.inner.get("pressure").as_::<f32>()
    }
}
impl PointerEvent {
    /// Getter of the `tangentialPressure` attribute.
    /// [`PointerEvent.tangentialPressure`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)
    pub fn tangential_pressure(&self) -> f32 {
        self.inner.get("tangentialPressure").as_::<f32>()
    }
}
impl PointerEvent {
    /// Getter of the `tiltX` attribute.
    /// [`PointerEvent.tiltX`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)
    pub fn tilt_x(&self) -> i32 {
        self.inner.get("tiltX").as_::<i32>()
    }
}
impl PointerEvent {
    /// Getter of the `tiltY` attribute.
    /// [`PointerEvent.tiltY`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)
    pub fn tilt_y(&self) -> i32 {
        self.inner.get("tiltY").as_::<i32>()
    }
}
impl PointerEvent {
    /// Getter of the `twist` attribute.
    /// [`PointerEvent.twist`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)
    pub fn twist(&self) -> i32 {
        self.inner.get("twist").as_::<i32>()
    }
}
impl PointerEvent {
    /// Getter of the `altitudeAngle` attribute.
    /// [`PointerEvent.altitudeAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/altitudeAngle)
    pub fn altitude_angle(&self) -> f64 {
        self.inner.get("altitudeAngle").as_::<f64>()
    }
}
impl PointerEvent {
    /// Getter of the `azimuthAngle` attribute.
    /// [`PointerEvent.azimuthAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/azimuthAngle)
    pub fn azimuth_angle(&self) -> f64 {
        self.inner.get("azimuthAngle").as_::<f64>()
    }
}
impl PointerEvent {
    /// Getter of the `pointerType` attribute.
    /// [`PointerEvent.pointerType`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)
    pub fn pointer_type(&self) -> JsString {
        self.inner.get("pointerType").as_::<JsString>()
    }
}
impl PointerEvent {
    /// Getter of the `isPrimary` attribute.
    /// [`PointerEvent.isPrimary`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl PointerEvent {
    /// Getter of the `persistentDeviceId` attribute.
    /// [`PointerEvent.persistentDeviceId`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/persistentDeviceId)
    pub fn persistent_device_id(&self) -> i32 {
        self.inner.get("persistentDeviceId").as_::<i32>()
    }
}
impl PointerEvent {
    /// The getCoalescedEvents method.
    /// [`PointerEvent.getCoalescedEvents`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)
    pub fn get_coalesced_events(&self) -> TypedArray<PointerEvent> {
        self.inner
            .call("getCoalescedEvents", &[])
            .as_::<TypedArray<PointerEvent>>()
    }
}
impl PointerEvent {
    /// The getPredictedEvents method.
    /// [`PointerEvent.getPredictedEvents`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getPredictedEvents)
    pub fn get_predicted_events(&self) -> TypedArray<PointerEvent> {
        self.inner
            .call("getPredictedEvents", &[])
            .as_::<TypedArray<PointerEvent>>()
    }
}
