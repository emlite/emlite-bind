use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyboardEvent {
    inner: UIEvent,
}
impl FromVal for KeyboardEvent {
    fn from_val(v: &emlite::Val) -> Self {
        KeyboardEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for KeyboardEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for KeyboardEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for KeyboardEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for KeyboardEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<KeyboardEvent> for emlite::Val {
    fn from(s: KeyboardEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(KeyboardEvent);

impl KeyboardEvent {
    pub fn new0(type_: jsbind::DOMString) -> KeyboardEvent {
        Self {
            inner: emlite::Val::global("KeyboardEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> KeyboardEvent {
        Self {
            inner: emlite::Val::global("KeyboardEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl KeyboardEvent {
    pub fn key(&self) -> jsbind::DOMString {
        self.inner.get("key").as_::<jsbind::DOMString>()
    }
}
impl KeyboardEvent {
    pub fn code(&self) -> jsbind::DOMString {
        self.inner.get("code").as_::<jsbind::DOMString>()
    }
}
impl KeyboardEvent {
    pub fn location(&self) -> u32 {
        self.inner.get("location").as_::<u32>()
    }
}
impl KeyboardEvent {
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn repeat(&self) -> bool {
        self.inner.get("repeat").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn is_composing(&self) -> bool {
        self.inner.get("isComposing").as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn get_modifier_state(&self, key_arg: jsbind::DOMString) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
impl KeyboardEvent {
    pub fn init_keyboard_event0(&self, type_arg: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initKeyboardEvent", &[type_arg.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event1(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call("initKeyboardEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event2(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event3(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event4(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event5(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
        location_arg: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                    location_arg.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event6(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
        location_arg: u32,
        ctrl_key: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                    location_arg.into(),
                    ctrl_key.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event7(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                    location_arg.into(),
                    ctrl_key.into(),
                    alt_key.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event8(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                    location_arg.into(),
                    ctrl_key.into(),
                    alt_key.into(),
                    shift_key.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_keyboard_event9(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Window,
        key_arg: jsbind::DOMString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    key_arg.into(),
                    location_arg.into(),
                    ctrl_key.into(),
                    alt_key.into(),
                    shift_key.into(),
                    meta_key.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl KeyboardEvent {
    pub fn char_code(&self) -> u32 {
        self.inner.get("charCode").as_::<u32>()
    }
}
impl KeyboardEvent {
    pub fn key_code(&self) -> u32 {
        self.inner.get("keyCode").as_::<u32>()
    }
}
