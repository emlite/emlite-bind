use super::*;




/// The RTCRtpStreamStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpStreamStats {
    inner: Any,
}

impl FromVal for RTCRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpStreamStats> for Any {
    fn from(s: RTCRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpStreamStats> for Any {
    fn from(s: &RTCRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpStreamStats {
    /// Getter of the `ssrc` attribute.
    pub fn ssrc(&self) -> u32 {
        self.inner.get("ssrc").as_::<u32>()
    }

    /// Setter of the `ssrc` attribute.
    pub fn set_ssrc(&mut self, value: u32) {
        self.inner.set("ssrc", value);
    }
}
impl RTCRtpStreamStats {
    /// Getter of the `kind` attribute.
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }

    /// Setter of the `kind` attribute.
    pub fn set_kind(&mut self, value: &JsString) {
        self.inner.set("kind", value);
    }
}
impl RTCRtpStreamStats {
    /// Getter of the `transportId` attribute.
    pub fn transport_id(&self) -> JsString {
        self.inner.get("transportId").as_::<JsString>()
    }

    /// Setter of the `transportId` attribute.
    pub fn set_transport_id(&mut self, value: &JsString) {
        self.inner.set("transportId", value);
    }
}
impl RTCRtpStreamStats {
    /// Getter of the `codecId` attribute.
    pub fn codec_id(&self) -> JsString {
        self.inner.get("codecId").as_::<JsString>()
    }

    /// Setter of the `codecId` attribute.
    pub fn set_codec_id(&mut self, value: &JsString) {
        self.inner.set("codecId", value);
    }
}
