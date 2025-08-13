use super::*;




/// The RTCCodecStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCCodecStats {
    inner: Any,
}

impl FromVal for RTCCodecStats {
    fn from_val(v: &Any) -> Self {
        RTCCodecStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCCodecStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCCodecStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCCodecStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCCodecStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCCodecStats> for Any {
    fn from(s: RTCCodecStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCCodecStats> for Any {
    fn from(s: &RTCCodecStats) -> Any {
        s.inner.clone()
    }
}

impl RTCCodecStats {
    /// Getter of the `payloadType` attribute.
    pub fn payload_type(&self) -> u32 {
        self.inner.get("payloadType").as_::<u32>()
    }

    /// Setter of the `payloadType` attribute.
    pub fn set_payload_type(&mut self, value: u32) {
        self.inner.set("payloadType", value);
    }
}
impl RTCCodecStats {
    /// Getter of the `transportId` attribute.
    pub fn transport_id(&self) -> JsString {
        self.inner.get("transportId").as_::<JsString>()
    }

    /// Setter of the `transportId` attribute.
    pub fn set_transport_id(&mut self, value: &JsString) {
        self.inner.set("transportId", value);
    }
}
impl RTCCodecStats {
    /// Getter of the `mimeType` attribute.
    pub fn mime_type(&self) -> JsString {
        self.inner.get("mimeType").as_::<JsString>()
    }

    /// Setter of the `mimeType` attribute.
    pub fn set_mime_type(&mut self, value: &JsString) {
        self.inner.set("mimeType", value);
    }
}
impl RTCCodecStats {
    /// Getter of the `clockRate` attribute.
    pub fn clock_rate(&self) -> u32 {
        self.inner.get("clockRate").as_::<u32>()
    }

    /// Setter of the `clockRate` attribute.
    pub fn set_clock_rate(&mut self, value: u32) {
        self.inner.set("clockRate", value);
    }
}
impl RTCCodecStats {
    /// Getter of the `channels` attribute.
    pub fn channels(&self) -> u32 {
        self.inner.get("channels").as_::<u32>()
    }

    /// Setter of the `channels` attribute.
    pub fn set_channels(&mut self, value: u32) {
        self.inner.set("channels", value);
    }
}
impl RTCCodecStats {
    /// Getter of the `sdpFmtpLine` attribute.
    pub fn sdp_fmtp_line(&self) -> JsString {
        self.inner.get("sdpFmtpLine").as_::<JsString>()
    }

    /// Setter of the `sdpFmtpLine` attribute.
    pub fn set_sdp_fmtp_line(&mut self, value: &JsString) {
        self.inner.set("sdpFmtpLine", value);
    }
}
