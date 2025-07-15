use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MIDIInput {
    type Target = MIDIPort;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MIDIInput {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIInput {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIInput> for emlite::Val {
    fn from(s: MIDIInput) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MIDIInput> for emlite::Val {
    fn from(s: &MIDIInput) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIInput);

impl MIDIInput {
    pub fn onmidimessage(&self) -> Any {
        self.inner.get("onmidimessage").as_::<Any>()
    }

    pub fn set_onmidimessage(&mut self, value: &Any) {
        self.inner.set("onmidimessage", value);
    }
}
