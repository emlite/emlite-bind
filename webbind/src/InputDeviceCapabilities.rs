use super::*;

/// The InputDeviceCapabilities class.
/// [`InputDeviceCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceCapabilities)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputDeviceCapabilities {
    inner: Any,
}
impl FromVal for InputDeviceCapabilities {
    fn from_val(v: &Any) -> Self {
        InputDeviceCapabilities {
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
impl core::ops::Deref for InputDeviceCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InputDeviceCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for InputDeviceCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InputDeviceCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InputDeviceCapabilities> for Any {
    fn from(s: InputDeviceCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InputDeviceCapabilities> for Any {
    fn from(s: &InputDeviceCapabilities) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(InputDeviceCapabilities);

impl InputDeviceCapabilities {
    /// The `new InputDeviceCapabilities(..)` constructor, creating a new InputDeviceCapabilities instance
    pub fn new0() -> InputDeviceCapabilities {
        Self {
            inner: Any::global("InputDeviceCapabilities").new(&[]).as_::<Any>(),
        }
    }

    /// The `new InputDeviceCapabilities(..)` constructor, creating a new InputDeviceCapabilities instance
    pub fn new1(device_init_dict: &Any) -> InputDeviceCapabilities {
        Self {
            inner: Any::global("InputDeviceCapabilities")
                .new(&[device_init_dict.into()])
                .as_::<Any>(),
        }
    }
}
impl InputDeviceCapabilities {
    /// Getter of the `firesTouchEvents` attribute.
    /// [`InputDeviceCapabilities.firesTouchEvents`](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceCapabilities/firesTouchEvents)
    pub fn fires_touch_events(&self) -> bool {
        self.inner.get("firesTouchEvents").as_::<bool>()
    }
}
impl InputDeviceCapabilities {
    /// Getter of the `pointerMovementScrolls` attribute.
    /// [`InputDeviceCapabilities.pointerMovementScrolls`](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceCapabilities/pointerMovementScrolls)
    pub fn pointer_movement_scrolls(&self) -> bool {
        self.inner.get("pointerMovementScrolls").as_::<bool>()
    }
}
