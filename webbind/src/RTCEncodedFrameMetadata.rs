use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedFrameMetadata {
    inner: Any,
}
impl FromVal for RTCEncodedFrameMetadata {
    fn from_val(v: &Any) -> Self {
        RTCEncodedFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCEncodedFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCEncodedFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCEncodedFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCEncodedFrameMetadata> for Any {
    fn from(s: RTCEncodedFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCEncodedFrameMetadata> for Any {
    fn from(s: &RTCEncodedFrameMetadata) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedFrameMetadata {
    pub fn synchronization_source(&self) -> u32 {
        self.inner.get("synchronizationSource").as_::<u32>()
    }

    pub fn set_synchronization_source(&mut self, value: u32) {
        self.inner.set("synchronizationSource", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn payload_type(&self) -> u8 {
        self.inner.get("payloadType").as_::<u8>()
    }

    pub fn set_payload_type(&mut self, value: u8) {
        self.inner.set("payloadType", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn contributing_sources(&self) -> TypedArray<u32> {
        self.inner
            .get("contributingSources")
            .as_::<TypedArray<u32>>()
    }

    pub fn set_contributing_sources(&mut self, value: TypedArray<u32>) {
        self.inner.set("contributingSources", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn rtp_timestamp(&self) -> u32 {
        self.inner.get("rtpTimestamp").as_::<u32>()
    }

    pub fn set_rtp_timestamp(&mut self, value: u32) {
        self.inner.set("rtpTimestamp", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn receive_time(&self) -> Any {
        self.inner.get("receiveTime").as_::<Any>()
    }

    pub fn set_receive_time(&mut self, value: &Any) {
        self.inner.set("receiveTime", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn capture_time(&self) -> Any {
        self.inner.get("captureTime").as_::<Any>()
    }

    pub fn set_capture_time(&mut self, value: &Any) {
        self.inner.set("captureTime", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn sender_capture_time_offset(&self) -> Any {
        self.inner.get("senderCaptureTimeOffset").as_::<Any>()
    }

    pub fn set_sender_capture_time_offset(&mut self, value: &Any) {
        self.inner.set("senderCaptureTimeOffset", value);
    }
}
impl RTCEncodedFrameMetadata {
    pub fn mime_type(&self) -> JsString {
        self.inner.get("mimeType").as_::<JsString>()
    }

    pub fn set_mime_type(&mut self, value: &JsString) {
        self.inner.set("mimeType", value);
    }
}
