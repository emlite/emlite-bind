use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCTrackEventInit {
    inner: Any,
}
impl FromVal for RTCTrackEventInit {
    fn from_val(v: &Any) -> Self {
        RTCTrackEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCTrackEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCTrackEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCTrackEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCTrackEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCTrackEventInit> for Any {
    fn from(s: RTCTrackEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCTrackEventInit> for Any {
    fn from(s: &RTCTrackEventInit) -> Any {
        s.inner.clone()
    }
}

impl RTCTrackEventInit {
    pub fn receiver(&self) -> RTCRtpReceiver {
        self.inner.get("receiver").as_::<RTCRtpReceiver>()
    }

    pub fn set_receiver(&mut self, value: &RTCRtpReceiver) {
        self.inner.set("receiver", value);
    }
}
impl RTCTrackEventInit {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }

    pub fn set_track(&mut self, value: &MediaStreamTrack) {
        self.inner.set("track", value);
    }
}
impl RTCTrackEventInit {
    pub fn streams(&self) -> TypedArray<MediaStream> {
        self.inner.get("streams").as_::<TypedArray<MediaStream>>()
    }

    pub fn set_streams(&mut self, value: &TypedArray<MediaStream>) {
        self.inner.set("streams", value);
    }
}
impl RTCTrackEventInit {
    pub fn transceiver(&self) -> RTCRtpTransceiver {
        self.inner.get("transceiver").as_::<RTCRtpTransceiver>()
    }

    pub fn set_transceiver(&mut self, value: &RTCRtpTransceiver) {
        self.inner.set("transceiver", value);
    }
}
