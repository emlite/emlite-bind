use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GamepadTouch {
    inner: emlite::Val,
}
impl FromVal for GamepadTouch {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadTouch { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GamepadTouch {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GamepadTouch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadTouch> for emlite::Val {
    fn from(s: GamepadTouch) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadTouch {
    pub fn touch_id(&self) -> u32 {
        self.inner.get("touchId").as_::<u32>()
    }

    pub fn set_touch_id(&mut self, value: u32) {
        self.inner.set("touchId", value);
    }
}
impl GamepadTouch {
    pub fn surface_id(&self) -> u8 {
        self.inner.get("surfaceId").as_::<u8>()
    }

    pub fn set_surface_id(&mut self, value: u8) {
        self.inner.set("surfaceId", value);
    }
}
impl GamepadTouch {
    pub fn position(&self) -> DOMPointReadOnly {
        self.inner.get("position").as_::<DOMPointReadOnly>()
    }

    pub fn set_position(&mut self, value: DOMPointReadOnly) {
        self.inner.set("position", value);
    }
}
impl GamepadTouch {
    pub fn surface_dimensions(&self) -> DOMRectReadOnly {
        self.inner.get("surfaceDimensions").as_::<DOMRectReadOnly>()
    }

    pub fn set_surface_dimensions(&mut self, value: DOMRectReadOnly) {
        self.inner.set("surfaceDimensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Gamepad {
    inner: emlite::Val,
}
impl FromVal for Gamepad {
    fn from_val(v: &emlite::Val) -> Self {
        Gamepad {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Gamepad {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Gamepad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Gamepad> for emlite::Val {
    fn from(s: Gamepad) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Gamepad {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl Gamepad {
    pub fn index(&self) -> i32 {
        self.inner.get("index").as_::<i32>()
    }
}
impl Gamepad {
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl Gamepad {
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
    }
}
impl Gamepad {
    pub fn mapping(&self) -> GamepadMappingType {
        self.inner.get("mapping").as_::<GamepadMappingType>()
    }
}
impl Gamepad {
    pub fn axes(&self) -> jsbind::FrozenArray<f64> {
        self.inner.get("axes").as_::<jsbind::FrozenArray<f64>>()
    }
}
impl Gamepad {
    pub fn buttons(&self) -> jsbind::FrozenArray<GamepadButton> {
        self.inner
            .get("buttons")
            .as_::<jsbind::FrozenArray<GamepadButton>>()
    }
}
impl Gamepad {
    pub fn touches(&self) -> jsbind::FrozenArray<GamepadTouch> {
        self.inner
            .get("touches")
            .as_::<jsbind::FrozenArray<GamepadTouch>>()
    }
}
impl Gamepad {
    pub fn vibration_actuator(&self) -> GamepadHapticActuator {
        self.inner
            .get("vibrationActuator")
            .as_::<GamepadHapticActuator>()
    }
}
impl Gamepad {
    pub fn hand(&self) -> GamepadHand {
        self.inner.get("hand").as_::<GamepadHand>()
    }
}
impl Gamepad {
    pub fn haptic_actuators(&self) -> jsbind::FrozenArray<GamepadHapticActuator> {
        self.inner
            .get("hapticActuators")
            .as_::<jsbind::FrozenArray<GamepadHapticActuator>>()
    }
}
impl Gamepad {
    pub fn pose(&self) -> GamepadPose {
        self.inner.get("pose").as_::<GamepadPose>()
    }
}
