use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCRtpScriptTransform {
    inner: emlite::Val,
}
impl FromVal for RTCRtpScriptTransform {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpScriptTransform {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpScriptTransform {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpScriptTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpScriptTransform> for emlite::Val {
    fn from(s: RTCRtpScriptTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpScriptTransform {
    pub fn new0(worker: Worker) -> RTCRtpScriptTransform {
        Self {
            inner: emlite::Val::global("RTCRtpScriptTransform")
                .new(&[worker.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(worker: Worker, options: jsbind::Any) -> RTCRtpScriptTransform {
        Self {
            inner: emlite::Val::global("RTCRtpScriptTransform")
                .new(&[worker.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(
        worker: Worker,
        options: jsbind::Any,
        transfer: jsbind::Sequence<jsbind::Object>,
    ) -> RTCRtpScriptTransform {
        Self {
            inner: emlite::Val::global("RTCRtpScriptTransform")
                .new(&[worker.into(), options.into(), transfer.into()])
                .as_::<emlite::Val>(),
        }
    }
}
