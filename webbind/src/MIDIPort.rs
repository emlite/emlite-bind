use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIPort {
    inner: EventTarget,
}
impl FromVal for MIDIPort {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIPort {
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
impl core::ops::Deref for MIDIPort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MIDIPort {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIPort {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIPort> for emlite::Val {
    fn from(s: MIDIPort) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MIDIPort);

impl MIDIPort {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl MIDIPort {
    pub fn manufacturer(&self) -> DOMString {
        self.inner.get("manufacturer").as_::<DOMString>()
    }
}
impl MIDIPort {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl MIDIPort {
    pub fn type_(&self) -> MIDIPortType {
        self.inner.get("type").as_::<MIDIPortType>()
    }
}
impl MIDIPort {
    pub fn version(&self) -> DOMString {
        self.inner.get("version").as_::<DOMString>()
    }
}
impl MIDIPort {
    pub fn state(&self) -> MIDIPortDeviceState {
        self.inner.get("state").as_::<MIDIPortDeviceState>()
    }
}
impl MIDIPort {
    pub fn connection(&self) -> MIDIPortConnectionState {
        self.inner
            .get("connection")
            .as_::<MIDIPortConnectionState>()
    }
}
impl MIDIPort {
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    pub fn set_onstatechange(&mut self, value: Any) {
        self.inner.set("onstatechange", value);
    }
}
impl MIDIPort {
    pub fn open(&self) -> Promise {
        self.inner.call("open", &[]).as_::<Promise>()
    }
}
impl MIDIPort {
    pub fn close(&self) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }
}
