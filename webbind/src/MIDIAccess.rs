use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIAccess {
    inner: EventTarget,
}
impl FromVal for MIDIAccess {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIAccess {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MIDIAccess {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MIDIAccess {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIAccess {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIAccess> for emlite::Val {
    fn from(s: MIDIAccess) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MIDIAccess> for emlite::Val {
    fn from(s: &MIDIAccess) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIAccess);

impl MIDIAccess {
    pub fn inputs(&self) -> MIDIInputMap {
        self.inner.get("inputs").as_::<MIDIInputMap>()
    }
}
impl MIDIAccess {
    pub fn outputs(&self) -> MIDIOutputMap {
        self.inner.get("outputs").as_::<MIDIOutputMap>()
    }
}
impl MIDIAccess {
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl MIDIAccess {
    pub fn sysex_enabled(&self) -> bool {
        self.inner.get("sysexEnabled").as_::<bool>()
    }
}
