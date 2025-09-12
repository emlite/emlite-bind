use super::*;

/// The GamepadEvent class.
/// [`GamepadEvent`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadEvent {
    inner: Event,
}

impl FromVal for GamepadEvent {
    fn from_val(v: &Any) -> Self {
        GamepadEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GamepadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GamepadEvent> for Any {
    fn from(s: GamepadEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadEvent> for Any {
    fn from(s: &GamepadEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GamepadEvent);

impl GamepadEvent {
    /// Getter of the `gamepad` attribute.
    /// [`GamepadEvent.gamepad`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)
    pub fn gamepad(&self) -> Gamepad {
        self.inner.get("gamepad").as_::<Gamepad>()
    }
}

impl GamepadEvent {
    /// The `new GamepadEvent(..)` constructor, creating a new GamepadEvent instance
    pub fn new(type_: &JsString, event_init_dict: &GamepadEventInit) -> GamepadEvent {
        Self {
            inner: Any::global("GamepadEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
