use super::*;

/// The Gamepad class.
/// [`Gamepad`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Gamepad {
    inner: Any,
}

impl FromVal for Gamepad {
    fn from_val(v: &Any) -> Self {
        Gamepad {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Gamepad {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Gamepad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Gamepad {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Gamepad {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Gamepad> for Any {
    fn from(s: Gamepad) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Gamepad> for Any {
    fn from(s: &Gamepad) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Gamepad);

impl Gamepad {
    /// Getter of the `id` attribute.
    /// [`Gamepad.id`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl Gamepad {
    /// Getter of the `index` attribute.
    /// [`Gamepad.index`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)
    pub fn index(&self) -> i32 {
        self.inner.get("index").as_::<i32>()
    }
}
impl Gamepad {
    /// Getter of the `connected` attribute.
    /// [`Gamepad.connected`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl Gamepad {
    /// Getter of the `timestamp` attribute.
    /// [`Gamepad.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl Gamepad {
    /// Getter of the `mapping` attribute.
    /// [`Gamepad.mapping`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)
    pub fn mapping(&self) -> GamepadMappingType {
        self.inner.get("mapping").as_::<GamepadMappingType>()
    }
}
impl Gamepad {
    /// Getter of the `axes` attribute.
    /// [`Gamepad.axes`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)
    pub fn axes(&self) -> TypedArray<f64> {
        self.inner.get("axes").as_::<TypedArray<f64>>()
    }
}
impl Gamepad {
    /// Getter of the `buttons` attribute.
    /// [`Gamepad.buttons`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)
    pub fn buttons(&self) -> TypedArray<GamepadButton> {
        self.inner.get("buttons").as_::<TypedArray<GamepadButton>>()
    }
}
impl Gamepad {
    /// Getter of the `touches` attribute.
    /// [`Gamepad.touches`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/touches)
    pub fn touches(&self) -> TypedArray<GamepadTouch> {
        self.inner.get("touches").as_::<TypedArray<GamepadTouch>>()
    }
}
impl Gamepad {
    /// Getter of the `vibrationActuator` attribute.
    /// [`Gamepad.vibrationActuator`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/vibrationActuator)
    pub fn vibration_actuator(&self) -> GamepadHapticActuator {
        self.inner
            .get("vibrationActuator")
            .as_::<GamepadHapticActuator>()
    }
}
impl Gamepad {
    /// Getter of the `hand` attribute.
    /// [`Gamepad.hand`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)
    pub fn hand(&self) -> GamepadHand {
        self.inner.get("hand").as_::<GamepadHand>()
    }
}
impl Gamepad {
    /// Getter of the `hapticActuators` attribute.
    /// [`Gamepad.hapticActuators`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)
    pub fn haptic_actuators(&self) -> TypedArray<GamepadHapticActuator> {
        self.inner
            .get("hapticActuators")
            .as_::<TypedArray<GamepadHapticActuator>>()
    }
}
impl Gamepad {
    /// Getter of the `pose` attribute.
    /// [`Gamepad.pose`](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)
    pub fn pose(&self) -> GamepadPose {
        self.inner.get("pose").as_::<GamepadPose>()
    }
}
