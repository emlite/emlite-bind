use super::*;

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
impl RTCRtpReceiver {
    /// The getCapabilities method.
    /// [`RTCRtpReceiver.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getCapabilities)
    pub fn get_capabilities(kind: &JsString) -> RTCRtpCapabilities {
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
    pub fn get_contributing_sources(&self) -> TypedArray<RTCRtpContributingSource> {
        self.inner
            .call("getContributingSources", &[])
            .as_::<TypedArray<RTCRtpContributingSource>>()
    }
}
impl RTCRtpReceiver {
    /// The getSynchronizationSources method.
    /// [`RTCRtpReceiver.getSynchronizationSources`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)
    pub fn get_synchronization_sources(&self) -> TypedArray<RTCRtpSynchronizationSource> {
        self.inner
            .call("getSynchronizationSources", &[])
            .as_::<TypedArray<RTCRtpSynchronizationSource>>()
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
