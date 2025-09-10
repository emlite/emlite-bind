use super::*;

/// The RTCRtpSender class.
/// [`RTCRtpSender`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSender {
    inner: Any,
}

impl FromVal for RTCRtpSender {
    fn from_val(v: &Any) -> Self {
        RTCRtpSender {
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

impl core::ops::Deref for RTCRtpSender {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpSender {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpSender {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCRtpSender> for Any {
    fn from(s: RTCRtpSender) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpSender> for Any {
    fn from(s: &RTCRtpSender) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCRtpSender);

impl RTCRtpSender {
    /// Getter of the `track` attribute.
    /// [`RTCRtpSender.track`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCRtpSender {
    /// Getter of the `transport` attribute.
    /// [`RTCRtpSender.transport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transport)
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCRtpSender {
    /// The getCapabilities method.
    /// [`RTCRtpSender.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getCapabilities)
    pub fn get_capabilities(kind: &JsString) -> RTCRtpCapabilities {
        Any::global("RTCRtpSender")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpSender {
    /// The setParameters method.
    /// [`RTCRtpSender.setParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    pub fn set_parameters0(&self, parameters: &RTCRtpSendParameters) -> Promise<Undefined> {
        self.inner
            .call("setParameters", &[parameters.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The setParameters method.
    /// [`RTCRtpSender.setParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    pub fn set_parameters1(
        &self,
        parameters: &RTCRtpSendParameters,
        set_parameter_options: &RTCSetParameterOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setParameters",
                &[parameters.into(), set_parameter_options.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// The getParameters method.
    /// [`RTCRtpSender.getParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)
    pub fn get_parameters(&self) -> RTCRtpSendParameters {
        self.inner
            .call("getParameters", &[])
            .as_::<RTCRtpSendParameters>()
    }
}
impl RTCRtpSender {
    /// The replaceTrack method.
    /// [`RTCRtpSender.replaceTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)
    pub fn replace_track(&self, with_track: &MediaStreamTrack) -> Promise<Undefined> {
        self.inner
            .call("replaceTrack", &[with_track.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// The setStreams method.
    /// [`RTCRtpSender.setStreams`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setStreams)
    pub fn set_streams(&self, streams: &MediaStream) -> Undefined {
        self.inner
            .call("setStreams", &[streams.into()])
            .as_::<Undefined>()
    }
}
impl RTCRtpSender {
    /// The getStats method.
    /// [`RTCRtpSender.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)
    pub fn get_stats(&self) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<RTCStatsReport>>()
    }
}
impl RTCRtpSender {
    /// Getter of the `transform` attribute.
    /// [`RTCRtpSender.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)
    pub fn transform(&self) -> Any {
        self.inner.get("transform").as_::<Any>()
    }

    /// Setter of the `transform` attribute.
    /// [`RTCRtpSender.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)
    pub fn set_transform(&mut self, value: &Any) {
        self.inner.set("transform", value);
    }
}
impl RTCRtpSender {
    /// The generateKeyFrame method.
    /// [`RTCRtpSender.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)
    pub fn generate_key_frame0(&self) -> Promise<Undefined> {
        self.inner
            .call("generateKeyFrame", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The generateKeyFrame method.
    /// [`RTCRtpSender.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)
    pub fn generate_key_frame1(&self, rids: &TypedArray<JsString>) -> Promise<Undefined> {
        self.inner
            .call("generateKeyFrame", &[rids.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// Getter of the `dtmf` attribute.
    /// [`RTCRtpSender.dtmf`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)
    pub fn dtmf(&self) -> RTCDTMFSender {
        self.inner.get("dtmf").as_::<RTCDTMFSender>()
    }
}
