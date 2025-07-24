use super::*;

/// The KeyFrameRequestEvent class.
/// [`KeyFrameRequestEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyFrameRequestEvent {
    inner: Event,
}
impl FromVal for KeyFrameRequestEvent {
    fn from_val(v: &Any) -> Self {
        KeyFrameRequestEvent {
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
impl core::ops::Deref for KeyFrameRequestEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for KeyFrameRequestEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for KeyFrameRequestEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for KeyFrameRequestEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<KeyFrameRequestEvent> for Any {
    fn from(s: KeyFrameRequestEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&KeyFrameRequestEvent> for Any {
    fn from(s: &KeyFrameRequestEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(KeyFrameRequestEvent);

impl KeyFrameRequestEvent {
    /// The `new KeyFrameRequestEvent(..)` constructor, creating a new KeyFrameRequestEvent instance
    pub fn new0(type_: &DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: Any::global("KeyFrameRequestEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new KeyFrameRequestEvent(..)` constructor, creating a new KeyFrameRequestEvent instance
    pub fn new1(type_: &DOMString, rid: &DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: Any::global("KeyFrameRequestEvent")
                .new(&[type_.into(), rid.into()])
                .as_::<Event>(),
        }
    }
}
impl KeyFrameRequestEvent {
    /// Getter of the `rid` attribute.
    /// [`KeyFrameRequestEvent.rid`](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/rid)
    pub fn rid(&self) -> DOMString {
        self.inner.get("rid").as_::<DOMString>()
    }
}
