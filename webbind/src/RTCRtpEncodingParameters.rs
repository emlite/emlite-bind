use super::*;




/// The RTCRtpEncodingParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpEncodingParameters {
    inner: Any,
}

impl FromVal for RTCRtpEncodingParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpEncodingParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpEncodingParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpEncodingParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpEncodingParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpEncodingParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpEncodingParameters> for Any {
    fn from(s: RTCRtpEncodingParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpEncodingParameters> for Any {
    fn from(s: &RTCRtpEncodingParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpEncodingParameters {
    /// Getter of the `active` attribute.
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }

    /// Setter of the `active` attribute.
    pub fn set_active(&mut self, value: bool) {
        self.inner.set("active", value);
    }
}
impl RTCRtpEncodingParameters {
    /// Getter of the `codec` attribute.
    pub fn codec(&self) -> RTCRtpCodec {
        self.inner.get("codec").as_::<RTCRtpCodec>()
    }

    /// Setter of the `codec` attribute.
    pub fn set_codec(&mut self, value: &RTCRtpCodec) {
        self.inner.set("codec", value);
    }
}
impl RTCRtpEncodingParameters {
    /// Getter of the `maxBitrate` attribute.
    pub fn max_bitrate(&self) -> u32 {
        self.inner.get("maxBitrate").as_::<u32>()
    }

    /// Setter of the `maxBitrate` attribute.
    pub fn set_max_bitrate(&mut self, value: u32) {
        self.inner.set("maxBitrate", value);
    }
}
impl RTCRtpEncodingParameters {
    /// Getter of the `maxFramerate` attribute.
    pub fn max_framerate(&self) -> f64 {
        self.inner.get("maxFramerate").as_::<f64>()
    }

    /// Setter of the `maxFramerate` attribute.
    pub fn set_max_framerate(&mut self, value: f64) {
        self.inner.set("maxFramerate", value);
    }
}
impl RTCRtpEncodingParameters {
    /// Getter of the `scaleResolutionDownBy` attribute.
    pub fn scale_resolution_down_by(&self) -> f64 {
        self.inner.get("scaleResolutionDownBy").as_::<f64>()
    }

    /// Setter of the `scaleResolutionDownBy` attribute.
    pub fn set_scale_resolution_down_by(&mut self, value: f64) {
        self.inner.set("scaleResolutionDownBy", value);
    }
}
