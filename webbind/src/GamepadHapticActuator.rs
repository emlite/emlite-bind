use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadEffectParameters {
    inner: emlite::Val,
}
impl FromVal for GamepadEffectParameters {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadEffectParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GamepadEffectParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GamepadEffectParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GamepadEffectParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GamepadEffectParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GamepadEffectParameters> for emlite::Val {
    fn from(s: GamepadEffectParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadEffectParameters {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }

}
impl GamepadEffectParameters {
    pub fn start_delay(&self) -> u64 {
        self.inner.get("startDelay").as_::<u64>()
    }

    pub fn set_start_delay(&mut self, value: u64) {
        self.inner.set("startDelay", value);
    }

}
impl GamepadEffectParameters {
    pub fn strong_magnitude(&self) -> f64 {
        self.inner.get("strongMagnitude").as_::<f64>()
    }

    pub fn set_strong_magnitude(&mut self, value: f64) {
        self.inner.set("strongMagnitude", value);
    }

}
impl GamepadEffectParameters {
    pub fn weak_magnitude(&self) -> f64 {
        self.inner.get("weakMagnitude").as_::<f64>()
    }

    pub fn set_weak_magnitude(&mut self, value: f64) {
        self.inner.set("weakMagnitude", value);
    }

}
impl GamepadEffectParameters {
    pub fn left_trigger(&self) -> f64 {
        self.inner.get("leftTrigger").as_::<f64>()
    }

    pub fn set_left_trigger(&mut self, value: f64) {
        self.inner.set("leftTrigger", value);
    }

}
impl GamepadEffectParameters {
    pub fn right_trigger(&self) -> f64 {
        self.inner.get("rightTrigger").as_::<f64>()
    }

    pub fn set_right_trigger(&mut self, value: f64) {
        self.inner.set("rightTrigger", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadHapticActuator {
    inner: emlite::Val,
}
impl FromVal for GamepadHapticActuator {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadHapticActuator { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GamepadHapticActuator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GamepadHapticActuator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GamepadHapticActuator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GamepadHapticActuator {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GamepadHapticActuator> for emlite::Val {
    fn from(s: GamepadHapticActuator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GamepadHapticActuator);


impl GamepadHapticActuator {
    pub fn effects(&self) -> FrozenArray<GamepadHapticEffectType> {
        self.inner.get("effects").as_::<FrozenArray<GamepadHapticEffectType>>()
    }

}
impl GamepadHapticActuator {
    pub fn play_effect0(&self, type_: GamepadHapticEffectType) -> Promise {
        self.inner.call("playEffect", &[type_.into(), ]).as_::<Promise>()
    }

    pub fn play_effect1(&self, type_: GamepadHapticEffectType, params: GamepadEffectParameters) -> Promise {
        self.inner.call("playEffect", &[type_.into(), params.into(), ]).as_::<Promise>()
    }

}
impl GamepadHapticActuator {
    pub fn reset(&self, ) -> Promise {
        self.inner.call("reset", &[]).as_::<Promise>()
    }

}
impl GamepadHapticActuator {
    pub fn pulse(&self, value: f64, duration: f64) -> Promise {
        self.inner.call("pulse", &[value.into(), duration.into(), ]).as_::<Promise>()
    }

}
