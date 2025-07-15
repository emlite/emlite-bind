use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for MessageEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MessageEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MessageEvent> for emlite::Val {
    fn from(s: MessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MessageEvent);

impl MessageEvent {
    pub fn new0(type_: DOMString) -> MessageEvent {
        Self {
            inner: emlite::Val::global("MessageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> MessageEvent {
        Self {
            inner: emlite::Val::global("MessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MessageEvent {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
impl MessageEvent {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl MessageEvent {
    pub fn last_event_id(&self) -> DOMString {
        self.inner.get("lastEventId").as_::<DOMString>()
    }
}
impl MessageEvent {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }
}
impl MessageEvent {
    pub fn ports(&self) -> FrozenArray<Any> {
        self.inner.get("ports").as_::<FrozenArray<Any>>()
    }
}
impl MessageEvent {
    pub fn init_message_event0(&self, type_: DOMString) -> Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into()])
            .as_::<Undefined>()
    }

    pub fn init_message_event1(&self, type_: DOMString, bubbles: bool) -> Undefined {
        self.inner
            .call("initMessageEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }

    pub fn init_message_event2(
        &self,
        type_: DOMString,
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

    pub fn init_message_event3(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        data: Any,
    ) -> Undefined {
        self.inner
            .call(
                "initMessageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_message_event4(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        data: Any,
        origin: USVString,
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

    pub fn init_message_event5(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        data: Any,
        origin: USVString,
        last_event_id: DOMString,
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

    pub fn init_message_event6(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        data: Any,
        origin: USVString,
        last_event_id: DOMString,
        source: Any,
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

    pub fn init_message_event7(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        data: Any,
        origin: USVString,
        last_event_id: DOMString,
        source: Any,
        ports: Sequence<Any>,
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
