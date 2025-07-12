use super::*;

#[derive(Clone, Debug)]
pub struct MIDIInput {
    inner: MIDIPort,
}
impl FromVal for MIDIInput {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIInput {
            inner: MIDIPort::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MIDIInput {
    type Target = MIDIPort;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MIDIInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MIDIInput> for emlite::Val {
    fn from(s: MIDIInput) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIInput {
    pub fn onmidimessage(&self) -> jsbind::Any {
        self.inner.get("onmidimessage").as_::<jsbind::Any>()
    }

    pub fn set_onmidimessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmidimessage", value);
    }
}
