use super::*;

/// The MessageEvent class.
/// [`MessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessageEvent {
    inner: Event,
}
impl FromVal for MessageEvent {
    fn from_val(v: &Any) -> Self {
        MessageEvent {
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
impl core::ops::Deref for MessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MessageEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MessageEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MessageEvent> for Any {
    fn from(s: MessageEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MessageEvent> for Any {
    fn from(s: &MessageEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MessageEvent);

impl MessageEvent {
    /// The `new MessageEvent(..)` constructor, creating a new MessageEvent instance
    pub fn new0(type_: &JsString) -> MessageEvent {
        Self {
            inner: Any::global("MessageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new MessageEvent(..)` constructor, creating a new MessageEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> MessageEvent {
        Self {
            inner: Any::global("MessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MessageEvent {
    /// Getter of the `data` attribute.
    /// [`MessageEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
impl MessageEvent {
    /// Getter of the `origin` attribute.
    /// [`MessageEvent.origin`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }
}
impl MessageEvent {
    /// Getter of the `lastEventId` attribute.
    /// [`MessageEvent.lastEventId`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)
    pub fn last_event_id(&self) -> JsString {
        self.inner.get("lastEventId").as_::<JsString>()
    }
}
impl MessageEvent {
    /// Getter of the `source` attribute.
    /// [`MessageEvent.source`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }
}
impl MessageEvent {
    /// Getter of the `ports` attribute.
    /// [`MessageEvent.ports`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)
    pub fn ports(&self) -> TypedArray<Any> {
        self.inner.get("ports").as_::<TypedArray<Any>>()
    }
}
impl MessageEvent {
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event0(&self, type_: &JsString) -> Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into()])
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event1(&self, type_: &JsString, bubbles: bool) -> Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event2(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event3(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), data.into()],
            )
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event4(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        data: &Any,
        origin: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    data.into(),
                    origin.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event5(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        data: &Any,
        origin: &JsString,
        last_event_id: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    data.into(),
                    origin.into(),
                    last_event_id.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event6(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        data: &Any,
        origin: &JsString,
        last_event_id: &JsString,
        source: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    data.into(),
                    origin.into(),
                    last_event_id.into(),
                    source.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMessageEvent method.
    /// [`MessageEvent.initMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    pub fn init_message_event7(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        data: &Any,
        origin: &JsString,
        last_event_id: &JsString,
        source: &Any,
        ports: &TypedArray<Any>,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    data.into(),
                    origin.into(),
                    last_event_id.into(),
                    source.into(),
                    ports.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
