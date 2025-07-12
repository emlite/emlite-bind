use super::*;

#[derive(Clone, Debug)]
pub struct KeyFrameRequestEvent {
    inner: Event,
}
impl FromVal for KeyFrameRequestEvent {
    fn from_val(v: &emlite::Val) -> Self {
        KeyFrameRequestEvent {
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
impl std::ops::Deref for KeyFrameRequestEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for KeyFrameRequestEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KeyFrameRequestEvent> for emlite::Val {
    fn from(s: KeyFrameRequestEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl KeyFrameRequestEvent {
    pub fn new0(type_: jsbind::DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: emlite::Val::global("KeyFrameRequestEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, rid: jsbind::DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: emlite::Val::global("KeyFrameRequestEvent")
                .new(&[type_.into(), rid.into()])
                .as_::<Event>(),
        }
    }
}
impl KeyFrameRequestEvent {
    pub fn rid(&self) -> jsbind::DOMString {
        self.inner.get("rid").as_::<jsbind::DOMString>()
    }
}
