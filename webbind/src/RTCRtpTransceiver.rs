use super::*;

#[derive(Clone, Debug)]
pub struct RTCRtpCodec {
    inner: emlite::Val,
}
impl FromVal for RTCRtpCodec {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpCodec { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCRtpCodec {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCRtpCodec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpCodec> for emlite::Val {
    fn from(s: RTCRtpCodec) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpCodec {
    pub fn mime_type(&self) -> jsbind::DOMString {
        self.inner.get("mimeType").as_::<jsbind::DOMString>()
    }

    pub fn set_mime_type(&mut self, value: jsbind::DOMString) {
        self.inner.set("mimeType", value);
    }
}
impl RTCRtpCodec {
    pub fn clock_rate(&self) -> u32 {
        self.inner.get("clockRate").as_::<u32>()
    }

    pub fn set_clock_rate(&mut self, value: u32) {
        self.inner.set("clockRate", value);
    }
}
impl RTCRtpCodec {
    pub fn channels(&self) -> u16 {
        self.inner.get("channels").as_::<u16>()
    }

    pub fn set_channels(&mut self, value: u16) {
        self.inner.set("channels", value);
    }
}
impl RTCRtpCodec {
    pub fn sdp_fmtp_line(&self) -> jsbind::DOMString {
        self.inner.get("sdpFmtpLine").as_::<jsbind::DOMString>()
    }

    pub fn set_sdp_fmtp_line(&mut self, value: jsbind::DOMString) {
        self.inner.set("sdpFmtpLine", value);
    }
}
#[derive(Clone, Debug)]
pub struct RTCRtpTransceiver {
    inner: emlite::Val,
}
impl FromVal for RTCRtpTransceiver {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpTransceiver {
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
impl std::ops::Deref for RTCRtpTransceiver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCRtpTransceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpTransceiver> for emlite::Val {
    fn from(s: RTCRtpTransceiver) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpTransceiver {
    pub fn mid(&self) -> jsbind::DOMString {
        self.inner.get("mid").as_::<jsbind::DOMString>()
    }
}
impl RTCRtpTransceiver {
    pub fn sender(&self) -> RTCRtpSender {
        self.inner.get("sender").as_::<RTCRtpSender>()
    }
}
impl RTCRtpTransceiver {
    pub fn receiver(&self) -> RTCRtpReceiver {
        self.inner.get("receiver").as_::<RTCRtpReceiver>()
    }
}
impl RTCRtpTransceiver {
    pub fn direction(&self) -> RTCRtpTransceiverDirection {
        self.inner
            .get("direction")
            .as_::<RTCRtpTransceiverDirection>()
    }

    pub fn set_direction(&mut self, value: RTCRtpTransceiverDirection) {
        self.inner.set("direction", value);
    }
}
impl RTCRtpTransceiver {
    pub fn current_direction(&self) -> RTCRtpTransceiverDirection {
        self.inner
            .get("currentDirection")
            .as_::<RTCRtpTransceiverDirection>()
    }
}
impl RTCRtpTransceiver {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl RTCRtpTransceiver {
    pub fn set_codec_preferences(
        &self,
        codecs: jsbind::Sequence<RTCRtpCodec>,
    ) -> jsbind::Undefined {
        self.inner
            .call("setCodecPreferences", &[codecs.into()])
            .as_::<jsbind::Undefined>()
    }
}
