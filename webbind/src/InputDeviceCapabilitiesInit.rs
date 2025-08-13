use super::*;




/// The InputDeviceCapabilitiesInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputDeviceCapabilitiesInit {
    inner: Any,
}

impl FromVal for InputDeviceCapabilitiesInit {
    fn from_val(v: &Any) -> Self {
        InputDeviceCapabilitiesInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InputDeviceCapabilitiesInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InputDeviceCapabilitiesInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InputDeviceCapabilitiesInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InputDeviceCapabilitiesInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InputDeviceCapabilitiesInit> for Any {
    fn from(s: InputDeviceCapabilitiesInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InputDeviceCapabilitiesInit> for Any {
    fn from(s: &InputDeviceCapabilitiesInit) -> Any {
        s.inner.clone()
    }
}

impl InputDeviceCapabilitiesInit {
    /// Getter of the `firesTouchEvents` attribute.
    pub fn fires_touch_events(&self) -> bool {
        self.inner.get("firesTouchEvents").as_::<bool>()
    }

    /// Setter of the `firesTouchEvents` attribute.
    pub fn set_fires_touch_events(&mut self, value: bool) {
        self.inner.set("firesTouchEvents", value);
    }
}
impl InputDeviceCapabilitiesInit {
    /// Getter of the `pointerMovementScrolls` attribute.
    pub fn pointer_movement_scrolls(&self) -> bool {
        self.inner.get("pointerMovementScrolls").as_::<bool>()
    }

    /// Setter of the `pointerMovementScrolls` attribute.
    pub fn set_pointer_movement_scrolls(&mut self, value: bool) {
        self.inner.set("pointerMovementScrolls", value);
    }
}
