use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpReceiveParameters {
    inner: Any,
}
impl FromVal for RTCRtpReceiveParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpReceiveParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpReceiveParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpReceiveParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpReceiveParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpReceiveParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpReceiveParameters> for Any {
    fn from(s: RTCRtpReceiveParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpReceiveParameters> for Any {
    fn from(s: &RTCRtpReceiveParameters) -> Any {
        s.inner.clone()
    }
}

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
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    pub fn set_timestamp(&mut self, value: &Any) {
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
#[repr(transparent)]
pub struct RTCRtpSynchronizationSource {
    inner: Any,
}
impl FromVal for RTCRtpSynchronizationSource {
    fn from_val(v: &Any) -> Self {
        RTCRtpSynchronizationSource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpSynchronizationSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSynchronizationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpSynchronizationSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpSynchronizationSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpSynchronizationSource> for Any {
    fn from(s: RTCRtpSynchronizationSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpSynchronizationSource> for Any {
    fn from(s: &RTCRtpSynchronizationSource) -> Any {
        s.inner.clone()
    }
}

/// The RTCRtpReceiver class.
/// [`RTCRtpReceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpReceiver {
    inner: Any,
}
impl FromVal for RTCRtpReceiver {
    fn from_val(v: &Any) -> Self {
        RTCRtpReceiver {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpReceiver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpReceiver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpReceiver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpReceiver> for Any {
    fn from(s: RTCRtpReceiver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpReceiver> for Any {
    fn from(s: &RTCRtpReceiver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCRtpReceiver);

impl RTCRtpReceiver {
    /// Getter of the `track` attribute.
    /// [`RTCRtpReceiver.track`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCRtpReceiver {
    /// Getter of the `transport` attribute.
    /// [`RTCRtpReceiver.transport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transport)
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCRtpReceiver {
    /// The getCapabilities method.
    /// [`RTCRtpReceiver.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getCapabilities)
    pub fn get_capabilities(kind: &str) -> RTCRtpCapabilities {
        Any::global("RTCRtpReceiver")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpReceiver {
    /// The getParameters method.
    /// [`RTCRtpReceiver.getParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getParameters)
    pub fn get_parameters(&self) -> RTCRtpReceiveParameters {
        self.inner
            .call("getParameters", &[])
            .as_::<RTCRtpReceiveParameters>()
    }
}
impl RTCRtpReceiver {
    /// The getContributingSources method.
    /// [`RTCRtpReceiver.getContributingSources`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getContributingSources)
    pub fn get_contributing_sources(&self) -> Sequence<RTCRtpContributingSource> {
        self.inner
            .call("getContributingSources", &[])
            .as_::<Sequence<RTCRtpContributingSource>>()
    }
}
impl RTCRtpReceiver {
    /// The getSynchronizationSources method.
    /// [`RTCRtpReceiver.getSynchronizationSources`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)
    pub fn get_synchronization_sources(&self) -> Sequence<RTCRtpSynchronizationSource> {
        self.inner
            .call("getSynchronizationSources", &[])
            .as_::<Sequence<RTCRtpSynchronizationSource>>()
    }
}
impl RTCRtpReceiver {
    /// The getStats method.
    /// [`RTCRtpReceiver.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getStats)
    pub fn get_stats(&self) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<RTCStatsReport>>()
    }
}
impl RTCRtpReceiver {
    /// Getter of the `jitterBufferTarget` attribute.
    /// [`RTCRtpReceiver.jitterBufferTarget`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)
    pub fn jitter_buffer_target(&self) -> Any {
        self.inner.get("jitterBufferTarget").as_::<Any>()
    }

    /// Setter of the `jitterBufferTarget` attribute.
    /// [`RTCRtpReceiver.jitterBufferTarget`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)
    pub fn set_jitter_buffer_target(&mut self, value: &Any) {
        self.inner.set("jitterBufferTarget", value);
    }
}
impl RTCRtpReceiver {
    /// Getter of the `transform` attribute.
    /// [`RTCRtpReceiver.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transform)
    pub fn transform(&self) -> Any {
        self.inner.get("transform").as_::<Any>()
    }

    /// Setter of the `transform` attribute.
    /// [`RTCRtpReceiver.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transform)
    pub fn set_transform(&mut self, value: &Any) {
        self.inner.set("transform", value);
    }
}
