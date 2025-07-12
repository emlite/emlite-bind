use super::*;

#[derive(Clone, Debug)]
pub struct MIDIMessageEvent {
    inner: Event,
}
impl FromVal for MIDIMessageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIMessageEvent {
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
impl std::ops::Deref for MIDIMessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MIDIMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MIDIMessageEvent> for emlite::Val {
    fn from(s: MIDIMessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIMessageEvent {
    pub fn new0(type_: jsbind::DOMString) -> MIDIMessageEvent {
        Self {
            inner: emlite::Val::global("MIDIMessageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> MIDIMessageEvent {
        Self {
            inner: emlite::Val::global("MIDIMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MIDIMessageEvent {
    pub fn data(&self) -> jsbind::Uint8Array {
        self.inner.get("data").as_::<jsbind::Uint8Array>()
    }
}
