use super::*;

/// The GamepadEffectParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadEffectParameters {
    inner: Any,
}

impl FromVal for GamepadEffectParameters {
    fn from_val(v: &Any) -> Self {
        GamepadEffectParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GamepadEffectParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadEffectParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadEffectParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadEffectParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GamepadEffectParameters> for Any {
    fn from(s: GamepadEffectParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadEffectParameters> for Any {
    fn from(s: &GamepadEffectParameters) -> Any {
        s.inner.clone()
    }
}

impl GamepadEffectParameters {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl GamepadEffectParameters {
    /// Getter of the `startDelay` attribute.
    pub fn start_delay(&self) -> u64 {
        self.inner.get("startDelay").as_::<u64>()
    }

    /// Setter of the `startDelay` attribute.
    pub fn set_start_delay(&mut self, value: u64) {
        self.inner.set("startDelay", value);
    }
}
impl GamepadEffectParameters {
    /// Getter of the `strongMagnitude` attribute.
    pub fn strong_magnitude(&self) -> f64 {
        self.inner.get("strongMagnitude").as_::<f64>()
    }

    /// Setter of the `strongMagnitude` attribute.
    pub fn set_strong_magnitude(&mut self, value: f64) {
        self.inner.set("strongMagnitude", value);
    }
}
impl GamepadEffectParameters {
    /// Getter of the `weakMagnitude` attribute.
    pub fn weak_magnitude(&self) -> f64 {
        self.inner.get("weakMagnitude").as_::<f64>()
    }

    /// Setter of the `weakMagnitude` attribute.
    pub fn set_weak_magnitude(&mut self, value: f64) {
        self.inner.set("weakMagnitude", value);
    }
}
impl GamepadEffectParameters {
    /// Getter of the `leftTrigger` attribute.
    pub fn left_trigger(&self) -> f64 {
        self.inner.get("leftTrigger").as_::<f64>()
    }

    /// Setter of the `leftTrigger` attribute.
    pub fn set_left_trigger(&mut self, value: f64) {
        self.inner.set("leftTrigger", value);
    }
}
impl GamepadEffectParameters {
    /// Getter of the `rightTrigger` attribute.
    pub fn right_trigger(&self) -> f64 {
        self.inner.get("rightTrigger").as_::<f64>()
    }

    /// Setter of the `rightTrigger` attribute.
    pub fn set_right_trigger(&mut self, value: f64) {
        self.inner.set("rightTrigger", value);
    }
}
