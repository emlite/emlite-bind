use super::*;




/// The RTCRtpContributingSource dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpContributingSource {
    inner: Any,
}

impl FromVal for RTCRtpContributingSource {
    fn from_val(v: &Any) -> Self {
        RTCRtpContributingSource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpContributingSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpContributingSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpContributingSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpContributingSource {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpContributingSource> for Any {
    fn from(s: RTCRtpContributingSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpContributingSource> for Any {
    fn from(s: &RTCRtpContributingSource) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpContributingSource {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl RTCRtpContributingSource {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> u32 {
        self.inner.get("source").as_::<u32>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: u32) {
        self.inner.set("source", value);
    }
}
impl RTCRtpContributingSource {
    /// Getter of the `audioLevel` attribute.
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    /// Setter of the `audioLevel` attribute.
    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
impl RTCRtpContributingSource {
    /// Getter of the `rtpTimestamp` attribute.
    pub fn rtp_timestamp(&self) -> u32 {
        self.inner.get("rtpTimestamp").as_::<u32>()
    }

    /// Setter of the `rtpTimestamp` attribute.
    pub fn set_rtp_timestamp(&mut self, value: u32) {
        self.inner.set("rtpTimestamp", value);
    }
}
