use super::*;

/// The AudioContext class.
/// [`AudioContext`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioContext {
    inner: BaseAudioContext,
}

impl FromVal for AudioContext {
    fn from_val(v: &Any) -> Self {
        AudioContext {
            inner: BaseAudioContext::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioContext {
    type Target = BaseAudioContext;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioContext> for Any {
    fn from(s: AudioContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioContext> for Any {
    fn from(s: &AudioContext) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioContext);

impl AudioContext {
    /// Getter of the `baseLatency` attribute.
    /// [`AudioContext.baseLatency`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/baseLatency)
    pub fn base_latency(&self) -> f64 {
        self.inner.get("baseLatency").as_::<f64>()
    }
}
impl AudioContext {
    /// Getter of the `outputLatency` attribute.
    /// [`AudioContext.outputLatency`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/outputLatency)
    pub fn output_latency(&self) -> f64 {
        self.inner.get("outputLatency").as_::<f64>()
    }
}
impl AudioContext {
    /// Getter of the `sinkId` attribute.
    /// [`AudioContext.sinkId`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sinkId)
    pub fn sink_id(&self) -> Any {
        self.inner.get("sinkId").as_::<Any>()
    }
}
impl AudioContext {
    /// Getter of the `onsinkchange` attribute.
    /// [`AudioContext.onsinkchange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onsinkchange)
    pub fn onsinkchange(&self) -> Any {
        self.inner.get("onsinkchange").as_::<Any>()
    }

    /// Setter of the `onsinkchange` attribute.
    /// [`AudioContext.onsinkchange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onsinkchange)
    pub fn set_onsinkchange(&mut self, value: &Any) {
        self.inner.set("onsinkchange", value);
    }
}
impl AudioContext {
    /// Getter of the `onerror` attribute.
    /// [`AudioContext.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`AudioContext.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}

impl AudioContext {
    /// The `new AudioContext(..)` constructor, creating a new AudioContext instance
    pub fn new() -> AudioContext {
        Self {
            inner: Any::global("AudioContext")
                .new(&[])
                .as_::<BaseAudioContext>(),
        }
    }
}

impl AudioContext {
    /// The `new AudioContext(..)` constructor, creating a new AudioContext instance
    pub fn new_with_context_options(context_options: &AudioContextOptions) -> AudioContext {
        Self {
            inner: Any::global("AudioContext")
                .new(&[context_options.into()])
                .as_::<BaseAudioContext>(),
        }
    }
}

impl AudioContext {
    /// The getOutputTimestamp method.
    /// [`AudioContext.getOutputTimestamp`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/getOutputTimestamp)
    pub fn get_output_timestamp(&self) -> AudioTimestamp {
        self.inner
            .call("getOutputTimestamp", &[])
            .as_::<AudioTimestamp>()
    }
}
impl AudioContext {
    /// The resume method.
    /// [`AudioContext.resume`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/resume)
    pub fn resume(&self) -> Promise<Undefined> {
        self.inner.call("resume", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioContext {
    /// The suspend method.
    /// [`AudioContext.suspend`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/suspend)
    pub fn suspend(&self) -> Promise<Undefined> {
        self.inner.call("suspend", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioContext {
    /// The close method.
    /// [`AudioContext.close`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/close)
    pub fn close(&self) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioContext {
    /// The setSinkId method.
    /// [`AudioContext.setSinkId`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/setSinkId)
    pub fn set_sink_id(&self, sink_id: &Any) -> Promise<Undefined> {
        self.inner
            .call("setSinkId", &[sink_id.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl AudioContext {
    /// The createMediaElementSource method.
    /// [`AudioContext.createMediaElementSource`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaElementSource)
    pub fn create_media_element_source(
        &self,
        media_element: &HTMLMediaElement,
    ) -> MediaElementAudioSourceNode {
        self.inner
            .call("createMediaElementSource", &[media_element.into()])
            .as_::<MediaElementAudioSourceNode>()
    }
}
impl AudioContext {
    /// The createMediaStreamSource method.
    /// [`AudioContext.createMediaStreamSource`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamSource)
    pub fn create_media_stream_source(
        &self,
        media_stream: &MediaStream,
    ) -> MediaStreamAudioSourceNode {
        self.inner
            .call("createMediaStreamSource", &[media_stream.into()])
            .as_::<MediaStreamAudioSourceNode>()
    }
}
impl AudioContext {
    /// The createMediaStreamTrackSource method.
    /// [`AudioContext.createMediaStreamTrackSource`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamTrackSource)
    pub fn create_media_stream_track_source(
        &self,
        media_stream_track: &MediaStreamTrack,
    ) -> MediaStreamTrackAudioSourceNode {
        self.inner
            .call("createMediaStreamTrackSource", &[media_stream_track.into()])
            .as_::<MediaStreamTrackAudioSourceNode>()
    }
}
impl AudioContext {
    /// The createMediaStreamDestination method.
    /// [`AudioContext.createMediaStreamDestination`](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination)
    pub fn create_media_stream_destination(&self) -> MediaStreamAudioDestinationNode {
        self.inner
            .call("createMediaStreamDestination", &[])
            .as_::<MediaStreamAudioDestinationNode>()
    }
}
