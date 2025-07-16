use super::*;

/// The ExtendableMessageEvent class.
/// [`ExtendableMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableMessageEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableMessageEvent {
    fn from_val(v: &Any) -> Self {
        ExtendableMessageEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ExtendableMessageEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ExtendableMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ExtendableMessageEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ExtendableMessageEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ExtendableMessageEvent> for Any {
    fn from(s: ExtendableMessageEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ExtendableMessageEvent> for Any {
    fn from(s: &ExtendableMessageEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ExtendableMessageEvent);

impl ExtendableMessageEvent {
    /// The `new ExtendableMessageEvent(..)` constructor, creating a new ExtendableMessageEvent instance
    pub fn new0(type_: &str) -> ExtendableMessageEvent {
        Self {
            inner: Any::global("ExtendableMessageEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    /// The `new ExtendableMessageEvent(..)` constructor, creating a new ExtendableMessageEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> ExtendableMessageEvent {
        Self {
            inner: Any::global("ExtendableMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ExtendableMessageEvent {
    /// Getter of the `data` attribute.
    /// [`ExtendableMessageEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
impl ExtendableMessageEvent {
    /// Getter of the `origin` attribute.
    /// [`ExtendableMessageEvent.origin`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }
}
impl ExtendableMessageEvent {
    /// Getter of the `lastEventId` attribute.
    /// [`ExtendableMessageEvent.lastEventId`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)
    pub fn last_event_id(&self) -> String {
        self.inner.get("lastEventId").as_::<String>()
    }
}
impl ExtendableMessageEvent {
    /// Getter of the `source` attribute.
    /// [`ExtendableMessageEvent.source`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }
}
impl ExtendableMessageEvent {
    /// Getter of the `ports` attribute.
    /// [`ExtendableMessageEvent.ports`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)
    pub fn ports(&self) -> FrozenArray<Any> {
        self.inner.get("ports").as_::<FrozenArray<Any>>()
    }
}
