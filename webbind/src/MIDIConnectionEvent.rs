use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIConnectionEvent {
    inner: Event,
}
impl FromVal for MIDIConnectionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIConnectionEvent {
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
impl core::ops::Deref for MIDIConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MIDIConnectionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIConnectionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIConnectionEvent> for emlite::Val {
    fn from(s: MIDIConnectionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MIDIConnectionEvent> for emlite::Val {
    fn from(s: &MIDIConnectionEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIConnectionEvent);

impl MIDIConnectionEvent {
    pub fn new0(type_: &str) -> MIDIConnectionEvent {
        Self {
            inner: emlite::Val::global("MIDIConnectionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> MIDIConnectionEvent {
        Self {
            inner: emlite::Val::global("MIDIConnectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MIDIConnectionEvent {
    pub fn port(&self) -> MIDIPort {
        self.inner.get("port").as_::<MIDIPort>()
    }
}
