use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioSession {
    inner: EventTarget,
}
impl FromVal for AudioSession {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSession {
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
impl core::ops::Deref for AudioSession {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSession> for emlite::Val {
    fn from(s: AudioSession) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSession {
    pub fn type_(&self) -> AudioSessionType {
        self.inner.get("type").as_::<AudioSessionType>()
    }

    pub fn set_type_(&mut self, value: AudioSessionType) {
        self.inner.set("type", value);
    }
}
impl AudioSession {
    pub fn state(&self) -> AudioSessionState {
        self.inner.get("state").as_::<AudioSessionState>()
    }
}
impl AudioSession {
    pub fn onstatechange(&self) -> jsbind::Any {
        self.inner.get("onstatechange").as_::<jsbind::Any>()
    }

    pub fn set_onstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onstatechange", value);
    }
}
