use super::*;

#[derive(Clone, Debug)]
pub struct MessageEvent {
    inner: Event,
}
impl FromVal for MessageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MessageEvent {
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
impl std::ops::Deref for MessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MessageEvent> for emlite::Val {
    fn from(s: MessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MessageEvent {
    pub fn new0(type_: jsbind::DOMString) -> MessageEvent {
        Self {
            inner: emlite::Val::global("MessageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> MessageEvent {
        Self {
            inner: emlite::Val::global("MessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MessageEvent {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl MessageEvent {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl MessageEvent {
    pub fn last_event_id(&self) -> jsbind::DOMString {
        self.inner.get("lastEventId").as_::<jsbind::DOMString>()
    }
}
impl MessageEvent {
    pub fn source(&self) -> jsbind::Any {
        self.inner.get("source").as_::<jsbind::Any>()
    }
}
impl MessageEvent {
    pub fn ports(&self) -> jsbind::FrozenArray<jsbind::Any> {
        self.inner
            .get("ports")
            .as_::<jsbind::FrozenArray<jsbind::Any>>()
    }
}
impl MessageEvent {
    pub fn init_message_event0(&self, type_: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event1(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into(), bubbles.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event2(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event3(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        data: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), data.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event4(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        data: jsbind::Any,
        origin: jsbind::USVString,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event5(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        data: jsbind::Any,
        origin: jsbind::USVString,
        last_event_id: jsbind::DOMString,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event6(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        data: jsbind::Any,
        origin: jsbind::USVString,
        last_event_id: jsbind::DOMString,
        source: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_message_event7(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        data: jsbind::Any,
        origin: jsbind::USVString,
        last_event_id: jsbind::DOMString,
        source: jsbind::Any,
        ports: jsbind::Sequence<jsbind::Any>,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
