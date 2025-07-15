use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpScriptTransformer {
    inner: EventTarget,
}
impl FromVal for RTCRtpScriptTransformer {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpScriptTransformer {
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
impl core::ops::Deref for RTCRtpScriptTransformer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpScriptTransformer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCRtpScriptTransformer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCRtpScriptTransformer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCRtpScriptTransformer> for emlite::Val {
    fn from(s: RTCRtpScriptTransformer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCRtpScriptTransformer> for emlite::Val {
    fn from(s: &RTCRtpScriptTransformer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCRtpScriptTransformer);

impl RTCRtpScriptTransformer {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn generate_key_frame0(&self) -> Promise {
        self.inner.call("generateKeyFrame", &[]).as_::<Promise>()
    }

    pub fn generate_key_frame1(&self, rid: DOMString) -> Promise {
        self.inner
            .call("generateKeyFrame", &[rid.into()])
            .as_::<Promise>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn send_key_frame_request(&self) -> Promise {
        self.inner.call("sendKeyFrameRequest", &[]).as_::<Promise>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn onkeyframerequest(&self) -> Any {
        self.inner.get("onkeyframerequest").as_::<Any>()
    }

    pub fn set_onkeyframerequest(&mut self, value: Any) {
        self.inner.set("onkeyframerequest", value);
    }
}
impl RTCRtpScriptTransformer {
    pub fn options(&self) -> Any {
        self.inner.get("options").as_::<Any>()
    }
}
