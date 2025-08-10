use super::*;

/// The KeyboardEvent class.
/// [`KeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyboardEvent {
    inner: UIEvent,
}
impl FromVal for KeyboardEvent {
    fn from_val(v: &Any) -> Self {
        KeyboardEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for KeyboardEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for KeyboardEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<KeyboardEvent> for Any {
    fn from(s: KeyboardEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&KeyboardEvent> for Any {
    fn from(s: &KeyboardEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(KeyboardEvent);

impl KeyboardEvent {
    /// The `new KeyboardEvent(..)` constructor, creating a new KeyboardEvent instance
    pub fn new0(type_: &JsString) -> KeyboardEvent {
        Self {
            inner: Any::global("KeyboardEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new KeyboardEvent(..)` constructor, creating a new KeyboardEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &KeyboardEventInit) -> KeyboardEvent {
        Self {
            inner: Any::global("KeyboardEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl KeyboardEvent {
    /// Getter of the `key` attribute.
    /// [`KeyboardEvent.key`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    pub fn key(&self) -> JsString {
        self.inner.get("key").as_::<JsString>()
    }
}
impl KeyboardEvent {
    /// Getter of the `code` attribute.
    /// [`KeyboardEvent.code`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)
    pub fn code(&self) -> JsString {
        self.inner.get("code").as_::<JsString>()
    }
}
impl KeyboardEvent {
    /// Getter of the `location` attribute.
    /// [`KeyboardEvent.location`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)
    pub fn location(&self) -> u32 {
        self.inner.get("location").as_::<u32>()
    }
}
impl KeyboardEvent {
    /// Getter of the `ctrlKey` attribute.
    /// [`KeyboardEvent.ctrlKey`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// Getter of the `shiftKey` attribute.
    /// [`KeyboardEvent.shiftKey`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// Getter of the `altKey` attribute.
    /// [`KeyboardEvent.altKey`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// Getter of the `metaKey` attribute.
    /// [`KeyboardEvent.metaKey`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// Getter of the `repeat` attribute.
    /// [`KeyboardEvent.repeat`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
    pub fn repeat(&self) -> bool {
        self.inner.get("repeat").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// Getter of the `isComposing` attribute.
    /// [`KeyboardEvent.isComposing`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)
    pub fn is_composing(&self) -> bool {
        self.inner.get("isComposing").as_::<bool>()
    }
}
impl KeyboardEvent {
    /// The getModifierState method.
    /// [`KeyboardEvent.getModifierState`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)
    pub fn get_modifier_state(&self, key_arg: &JsString) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
impl KeyboardEvent {
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event0(&self, type_arg: &JsString) -> Undefined {
        self.inner
            .call("initKeyboardEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event1(&self, type_arg: &JsString, bubbles_arg: bool) -> Undefined {
        self.inner
            .call("initKeyboardEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event2(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initKeyboardEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event3(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event4(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event5(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
        location_arg: u32,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event6(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
        location_arg: u32,
        ctrl_key: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event7(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event8(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The initKeyboardEvent method.
    /// [`KeyboardEvent.initKeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)
    pub fn init_keyboard_event9(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        key_arg: &JsString,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl KeyboardEvent {
    /// Getter of the `charCode` attribute.
    /// [`KeyboardEvent.charCode`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/charCode)
    pub fn char_code(&self) -> u32 {
        self.inner.get("charCode").as_::<u32>()
    }
}
impl KeyboardEvent {
    /// Getter of the `keyCode` attribute.
    /// [`KeyboardEvent.keyCode`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode)
    pub fn key_code(&self) -> u32 {
        self.inner.get("keyCode").as_::<u32>()
    }
}
