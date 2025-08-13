use super::*;




/// The GamepadTouch dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadTouch {
    inner: Any,
}

impl FromVal for GamepadTouch {
    fn from_val(v: &Any) -> Self {
        GamepadTouch { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GamepadTouch {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadTouch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadTouch {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadTouch {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GamepadTouch> for Any {
    fn from(s: GamepadTouch) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadTouch> for Any {
    fn from(s: &GamepadTouch) -> Any {
        s.inner.clone()
    }
}

impl GamepadTouch {
    /// Getter of the `touchId` attribute.
    pub fn touch_id(&self) -> u32 {
        self.inner.get("touchId").as_::<u32>()
    }

    /// Setter of the `touchId` attribute.
    pub fn set_touch_id(&mut self, value: u32) {
        self.inner.set("touchId", value);
    }
}
impl GamepadTouch {
    /// Getter of the `surfaceId` attribute.
    pub fn surface_id(&self) -> u8 {
        self.inner.get("surfaceId").as_::<u8>()
    }

    /// Setter of the `surfaceId` attribute.
    pub fn set_surface_id(&mut self, value: u8) {
        self.inner.set("surfaceId", value);
    }
}
impl GamepadTouch {
    /// Getter of the `position` attribute.
    pub fn position(&self) -> DOMPointReadOnly {
        self.inner.get("position").as_::<DOMPointReadOnly>()
    }

    /// Setter of the `position` attribute.
    pub fn set_position(&mut self, value: &DOMPointReadOnly) {
        self.inner.set("position", value);
    }
}
impl GamepadTouch {
    /// Getter of the `surfaceDimensions` attribute.
    pub fn surface_dimensions(&self) -> DOMRectReadOnly {
        self.inner.get("surfaceDimensions").as_::<DOMRectReadOnly>()
    }

    /// Setter of the `surfaceDimensions` attribute.
    pub fn set_surface_dimensions(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("surfaceDimensions", value);
    }
}
