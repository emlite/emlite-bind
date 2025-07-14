use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCTransformEvent {
    inner: Event,
}
impl FromVal for RTCTransformEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCTransformEvent {
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
impl core::ops::Deref for RTCTransformEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCTransformEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCTransformEvent> for emlite::Val {
    fn from(s: RTCTransformEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCTransformEvent {
    pub fn transformer(&self) -> RTCRtpScriptTransformer {
        self.inner
            .get("transformer")
            .as_::<RTCRtpScriptTransformer>()
    }
}
