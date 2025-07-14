use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<RTCRtpScriptTransformer> for emlite::Val {
    fn from(s: RTCRtpScriptTransformer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpScriptTransformer {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn generate_key_frame0(&self) -> jsbind::Promise {
        self.inner
            .call("generateKeyFrame", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn generate_key_frame1(&self, rid: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("generateKeyFrame", &[rid.into()])
            .as_::<jsbind::Promise>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn send_key_frame_request(&self) -> jsbind::Promise {
        self.inner
            .call("sendKeyFrameRequest", &[])
            .as_::<jsbind::Promise>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl RTCRtpScriptTransformer {
    pub fn onkeyframerequest(&self) -> jsbind::Any {
        self.inner.get("onkeyframerequest").as_::<jsbind::Any>()
    }

    pub fn set_onkeyframerequest(&mut self, value: jsbind::Any) {
        self.inner.set("onkeyframerequest", value);
    }
}
impl RTCRtpScriptTransformer {
    pub fn options(&self) -> jsbind::Any {
        self.inner.get("options").as_::<jsbind::Any>()
    }
}
