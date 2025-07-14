use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MIDIMessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MIDIMessageEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIMessageEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIMessageEvent> for emlite::Val {
    fn from(s: MIDIMessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MIDIMessageEvent);

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
