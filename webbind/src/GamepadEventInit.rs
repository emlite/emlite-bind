use super::*;




/// The GamepadEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadEventInit {
    inner: Any,
}

impl FromVal for GamepadEventInit {
    fn from_val(v: &Any) -> Self {
        GamepadEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GamepadEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GamepadEventInit> for Any {
    fn from(s: GamepadEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadEventInit> for Any {
    fn from(s: &GamepadEventInit) -> Any {
        s.inner.clone()
    }
}

impl GamepadEventInit {
    /// Getter of the `gamepad` attribute.
    pub fn gamepad(&self) -> Gamepad {
        self.inner.get("gamepad").as_::<Gamepad>()
    }

    /// Setter of the `gamepad` attribute.
    pub fn set_gamepad(&mut self, value: &Gamepad) {
        self.inner.set("gamepad", value);
    }
}
