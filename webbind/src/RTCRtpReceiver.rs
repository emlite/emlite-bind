use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCRtpReceiveParameters {
    inner: emlite::Val,
}
impl FromVal for RTCRtpReceiveParameters {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpReceiveParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpReceiveParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpReceiveParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpReceiveParameters> for emlite::Val {
    fn from(s: RTCRtpReceiveParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCRtpContributingSource {
    inner: emlite::Val,
}
impl FromVal for RTCRtpContributingSource {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpContributingSource { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpContributingSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpContributingSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpContributingSource> for emlite::Val {
    fn from(s: RTCRtpContributingSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpContributingSource {
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
    }

    pub fn set_timestamp(&mut self, value: jsbind::Any) {
        self.inner.set("timestamp", value);
    }
}
impl RTCRtpContributingSource {
    pub fn source(&self) -> u32 {
        self.inner.get("source").as_::<u32>()
    }

    pub fn set_source(&mut self, value: u32) {
        self.inner.set("source", value);
    }
}
impl RTCRtpContributingSource {
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
impl RTCRtpContributingSource {
    pub fn rtp_timestamp(&self) -> u32 {
        self.inner.get("rtpTimestamp").as_::<u32>()
    }

    pub fn set_rtp_timestamp(&mut self, value: u32) {
        self.inner.set("rtpTimestamp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCRtpSynchronizationSource {
    inner: emlite::Val,
}
impl FromVal for RTCRtpSynchronizationSource {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpSynchronizationSource { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpSynchronizationSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSynchronizationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpSynchronizationSource> for emlite::Val {
    fn from(s: RTCRtpSynchronizationSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCRtpReceiver {
    inner: emlite::Val,
}
impl FromVal for RTCRtpReceiver {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpReceiver {
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
impl core::ops::Deref for RTCRtpReceiver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpReceiver> for emlite::Val {
    fn from(s: RTCRtpReceiver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpReceiver {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCRtpReceiver {
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCRtpReceiver {
    pub fn get_capabilities(kind: jsbind::DOMString) -> RTCRtpCapabilities {
        emlite::Val::global("rtcrtpreceiver")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpReceiver {
    pub fn get_parameters(&self) -> RTCRtpReceiveParameters {
        self.inner
            .call("getParameters", &[])
            .as_::<RTCRtpReceiveParameters>()
    }
}
impl RTCRtpReceiver {
    pub fn get_contributing_sources(&self) -> jsbind::Sequence<RTCRtpContributingSource> {
        self.inner
            .call("getContributingSources", &[])
            .as_::<jsbind::Sequence<RTCRtpContributingSource>>()
    }
}
impl RTCRtpReceiver {
    pub fn get_synchronization_sources(&self) -> jsbind::Sequence<RTCRtpSynchronizationSource> {
        self.inner
            .call("getSynchronizationSources", &[])
            .as_::<jsbind::Sequence<RTCRtpSynchronizationSource>>()
    }
}
impl RTCRtpReceiver {
    pub fn get_stats(&self) -> jsbind::Promise {
        self.inner.call("getStats", &[]).as_::<jsbind::Promise>()
    }
}
impl RTCRtpReceiver {
    pub fn jitter_buffer_target(&self) -> jsbind::Any {
        self.inner.get("jitterBufferTarget").as_::<jsbind::Any>()
    }

    pub fn set_jitter_buffer_target(&mut self, value: jsbind::Any) {
        self.inner.set("jitterBufferTarget", value);
    }
}
impl RTCRtpReceiver {
    pub fn transform(&self) -> jsbind::Any {
        self.inner.get("transform").as_::<jsbind::Any>()
    }

    pub fn set_transform(&mut self, value: jsbind::Any) {
        self.inner.set("transform", value);
    }
}
