use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDTMFSender {
    inner: EventTarget,
}
impl FromVal for RTCDTMFSender {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDTMFSender {
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
impl core::ops::Deref for RTCDTMFSender {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDTMFSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCDTMFSender {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCDTMFSender {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCDTMFSender> for emlite::Val {
    fn from(s: RTCDTMFSender) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCDTMFSender);

impl RTCDTMFSender {
    pub fn insert_dtmf0(&self, tones: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("insertDTMF", &[tones.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn insert_dtmf1(&self, tones: jsbind::DOMString, duration: u32) -> jsbind::Undefined {
        self.inner
            .call("insertDTMF", &[tones.into(), duration.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn insert_dtmf2(
        &self,
        tones: jsbind::DOMString,
        duration: u32,
        inter_tone_gap: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "insertDTMF",
                &[tones.into(), duration.into(), inter_tone_gap.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl RTCDTMFSender {
    pub fn ontonechange(&self) -> jsbind::Any {
        self.inner.get("ontonechange").as_::<jsbind::Any>()
    }

    pub fn set_ontonechange(&mut self, value: jsbind::Any) {
        self.inner.set("ontonechange", value);
    }
}
impl RTCDTMFSender {
    pub fn can_insert_dtmf(&self) -> bool {
        self.inner.get("canInsertDTMF").as_::<bool>()
    }
}
impl RTCDTMFSender {
    pub fn tone_buffer(&self) -> jsbind::DOMString {
        self.inner.get("toneBuffer").as_::<jsbind::DOMString>()
    }
}
