use super::*;




/// The RTCRtpCodec dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCodec {
    inner: Any,
}

impl FromVal for RTCRtpCodec {
    fn from_val(v: &Any) -> Self {
        RTCRtpCodec { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpCodec {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpCodec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpCodec {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpCodec {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpCodec> for Any {
    fn from(s: RTCRtpCodec) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpCodec> for Any {
    fn from(s: &RTCRtpCodec) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpCodec {
    /// Getter of the `mimeType` attribute.
    pub fn mime_type(&self) -> JsString {
        self.inner.get("mimeType").as_::<JsString>()
    }

    /// Setter of the `mimeType` attribute.
    pub fn set_mime_type(&mut self, value: &JsString) {
        self.inner.set("mimeType", value);
    }
}
impl RTCRtpCodec {
    /// Getter of the `clockRate` attribute.
    pub fn clock_rate(&self) -> u32 {
        self.inner.get("clockRate").as_::<u32>()
    }

    /// Setter of the `clockRate` attribute.
    pub fn set_clock_rate(&mut self, value: u32) {
        self.inner.set("clockRate", value);
    }
}
impl RTCRtpCodec {
    /// Getter of the `channels` attribute.
    pub fn channels(&self) -> u16 {
        self.inner.get("channels").as_::<u16>()
    }

    /// Setter of the `channels` attribute.
    pub fn set_channels(&mut self, value: u16) {
        self.inner.set("channels", value);
    }
}
impl RTCRtpCodec {
    /// Getter of the `sdpFmtpLine` attribute.
    pub fn sdp_fmtp_line(&self) -> JsString {
        self.inner.get("sdpFmtpLine").as_::<JsString>()
    }

    /// Setter of the `sdpFmtpLine` attribute.
    pub fn set_sdp_fmtp_line(&mut self, value: &JsString) {
        self.inner.set("sdpFmtpLine", value);
    }
}
