use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadEvent {
    inner: Event,
}
impl FromVal for GamepadEvent {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for GamepadEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GamepadEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GamepadEvent> for emlite::Val {
    fn from(s: GamepadEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GamepadEvent);

impl GamepadEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> GamepadEvent {
        Self {
            inner: emlite::Val::global("GamepadEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl GamepadEvent {
    pub fn gamepad(&self) -> Gamepad {
        self.inner.get("gamepad").as_::<Gamepad>()
    }
}
