use super::*;

/// The TouchEvent class.
/// [`TouchEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TouchEvent {
    inner: UIEvent,
}
impl FromVal for TouchEvent {
    fn from_val(v: &Any) -> Self {
        TouchEvent {
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
impl core::ops::Deref for TouchEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TouchEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TouchEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TouchEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TouchEvent> for Any {
    fn from(s: TouchEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TouchEvent> for Any {
    fn from(s: &TouchEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TouchEvent);

impl TouchEvent {
    /// The `new TouchEvent(..)` constructor, creating a new TouchEvent instance
    pub fn new0(type_: &JsString) -> TouchEvent {
        Self {
            inner: Any::global("TouchEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new TouchEvent(..)` constructor, creating a new TouchEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> TouchEvent {
        Self {
            inner: Any::global("TouchEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl TouchEvent {
    /// Getter of the `touches` attribute.
    /// [`TouchEvent.touches`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)
    pub fn touches(&self) -> TouchList {
        self.inner.get("touches").as_::<TouchList>()
    }
}
impl TouchEvent {
    /// Getter of the `targetTouches` attribute.
    /// [`TouchEvent.targetTouches`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)
    pub fn target_touches(&self) -> TouchList {
        self.inner.get("targetTouches").as_::<TouchList>()
    }
}
impl TouchEvent {
    /// Getter of the `changedTouches` attribute.
    /// [`TouchEvent.changedTouches`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)
    pub fn changed_touches(&self) -> TouchList {
        self.inner.get("changedTouches").as_::<TouchList>()
    }
}
impl TouchEvent {
    /// Getter of the `altKey` attribute.
    /// [`TouchEvent.altKey`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }
}
impl TouchEvent {
    /// Getter of the `metaKey` attribute.
    /// [`TouchEvent.metaKey`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }
}
impl TouchEvent {
    /// Getter of the `ctrlKey` attribute.
    /// [`TouchEvent.ctrlKey`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }
}
impl TouchEvent {
    /// Getter of the `shiftKey` attribute.
    /// [`TouchEvent.shiftKey`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }
}
impl TouchEvent {
    /// The getModifierState method.
    /// [`TouchEvent.getModifierState`](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/getModifierState)
    pub fn get_modifier_state(&self, key_arg: &JsString) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
