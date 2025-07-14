use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExtendableMessageEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableMessageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ExtendableMessageEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<ExtendableMessageEvent> for emlite::Val {
    fn from(s: ExtendableMessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ExtendableMessageEvent {
    pub fn new0(type_: jsbind::DOMString) -> ExtendableMessageEvent {
        Self {
            inner: emlite::Val::global("ExtendableMessageEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> ExtendableMessageEvent {
        Self {
            inner: emlite::Val::global("ExtendableMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ExtendableMessageEvent {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl ExtendableMessageEvent {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl ExtendableMessageEvent {
    pub fn last_event_id(&self) -> jsbind::DOMString {
        self.inner.get("lastEventId").as_::<jsbind::DOMString>()
    }
}
impl ExtendableMessageEvent {
    pub fn source(&self) -> jsbind::Any {
        self.inner.get("source").as_::<jsbind::Any>()
    }
}
impl ExtendableMessageEvent {
    pub fn ports(&self) -> jsbind::FrozenArray<jsbind::Any> {
        self.inner
            .get("ports")
            .as_::<jsbind::FrozenArray<jsbind::Any>>()
    }
}
