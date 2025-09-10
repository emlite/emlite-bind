use super::*;

/// The RTCRtpTransceiverInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpTransceiverInit {
    inner: Any,
}

impl FromVal for RTCRtpTransceiverInit {
    fn from_val(v: &Any) -> Self {
        RTCRtpTransceiverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpTransceiverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpTransceiverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpTransceiverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpTransceiverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCRtpTransceiverInit> for Any {
    fn from(s: RTCRtpTransceiverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpTransceiverInit> for Any {
    fn from(s: &RTCRtpTransceiverInit) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpTransceiverInit {
    /// Getter of the `direction` attribute.
    pub fn direction(&self) -> RTCRtpTransceiverDirection {
        self.inner
            .get("direction")
            .as_::<RTCRtpTransceiverDirection>()
    }

    /// Setter of the `direction` attribute.
    pub fn set_direction(&mut self, value: &RTCRtpTransceiverDirection) {
        self.inner.set("direction", value);
    }
}
impl RTCRtpTransceiverInit {
    /// Getter of the `streams` attribute.
    pub fn streams(&self) -> TypedArray<MediaStream> {
        self.inner.get("streams").as_::<TypedArray<MediaStream>>()
    }

    /// Setter of the `streams` attribute.
    pub fn set_streams(&mut self, value: &TypedArray<MediaStream>) {
        self.inner.set("streams", value);
    }
}
impl RTCRtpTransceiverInit {
    /// Getter of the `sendEncodings` attribute.
    pub fn send_encodings(&self) -> TypedArray<RTCRtpEncodingParameters> {
        self.inner
            .get("sendEncodings")
            .as_::<TypedArray<RTCRtpEncodingParameters>>()
    }

    /// Setter of the `sendEncodings` attribute.
    pub fn set_send_encodings(&mut self, value: &TypedArray<RTCRtpEncodingParameters>) {
        self.inner.set("sendEncodings", value);
    }
}
