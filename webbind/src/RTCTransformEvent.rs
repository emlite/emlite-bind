use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for RTCTransformEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCTransformEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(RTCTransformEvent);

impl RTCTransformEvent {
    pub fn transformer(&self) -> RTCRtpScriptTransformer {
        self.inner
            .get("transformer")
            .as_::<RTCRtpScriptTransformer>()
    }
}
