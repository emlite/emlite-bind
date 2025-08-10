use super::*;

/// The GamepadButton class.
/// [`GamepadButton`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadButton {
    inner: Any,
}

impl FromVal for GamepadButton {
    fn from_val(v: &Any) -> Self {
        GamepadButton {
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

impl core::ops::Deref for GamepadButton {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadButton {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadButton {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GamepadButton> for Any {
    fn from(s: GamepadButton) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadButton> for Any {
    fn from(s: &GamepadButton) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GamepadButton);

impl GamepadButton {
    /// Getter of the `pressed` attribute.
    /// [`GamepadButton.pressed`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)
    pub fn pressed(&self) -> bool {
        self.inner.get("pressed").as_::<bool>()
    }
}
impl GamepadButton {
    /// Getter of the `touched` attribute.
    /// [`GamepadButton.touched`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/touched)
    pub fn touched(&self) -> bool {
        self.inner.get("touched").as_::<bool>()
    }
}
impl GamepadButton {
    /// Getter of the `value` attribute.
    /// [`GamepadButton.value`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
