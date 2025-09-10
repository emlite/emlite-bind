use super::*;

/// The GamepadHapticActuator class.
/// [`GamepadHapticActuator`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadHapticActuator {
    inner: Any,
}

impl FromVal for GamepadHapticActuator {
    fn from_val(v: &Any) -> Self {
        GamepadHapticActuator {
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

impl core::ops::Deref for GamepadHapticActuator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadHapticActuator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadHapticActuator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadHapticActuator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GamepadHapticActuator> for Any {
    fn from(s: GamepadHapticActuator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadHapticActuator> for Any {
    fn from(s: &GamepadHapticActuator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GamepadHapticActuator);

impl GamepadHapticActuator {
    /// Getter of the `effects` attribute.
    /// [`GamepadHapticActuator.effects`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/effects)
    pub fn effects(&self) -> TypedArray<GamepadHapticEffectType> {
        self.inner
            .get("effects")
            .as_::<TypedArray<GamepadHapticEffectType>>()
    }
}
impl GamepadHapticActuator {
    /// The playEffect method.
    /// [`GamepadHapticActuator.playEffect`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)
    pub fn play_effect0(&self, type_: &GamepadHapticEffectType) -> Promise<GamepadHapticsResult> {
        self.inner
            .call("playEffect", &[type_.into()])
            .as_::<Promise<GamepadHapticsResult>>()
    }
    /// The playEffect method.
    /// [`GamepadHapticActuator.playEffect`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)
    pub fn play_effect1(
        &self,
        type_: &GamepadHapticEffectType,
        params: &GamepadEffectParameters,
    ) -> Promise<GamepadHapticsResult> {
        self.inner
            .call("playEffect", &[type_.into(), params.into()])
            .as_::<Promise<GamepadHapticsResult>>()
    }
}
impl GamepadHapticActuator {
    /// The reset method.
    /// [`GamepadHapticActuator.reset`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/reset)
    pub fn reset(&self) -> Promise<GamepadHapticsResult> {
        self.inner
            .call("reset", &[])
            .as_::<Promise<GamepadHapticsResult>>()
    }
}
impl GamepadHapticActuator {
    /// The pulse method.
    /// [`GamepadHapticActuator.pulse`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)
    pub fn pulse(&self, value: f64, duration: f64) -> Promise<bool> {
        self.inner
            .call("pulse", &[value.into(), duration.into()])
            .as_::<Promise<bool>>()
    }
}
