use super::*;

/// The MediaSource class.
/// [`MediaSource`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaSource {
    inner: EventTarget,
}
impl FromVal for MediaSource {
    fn from_val(v: &Any) -> Self {
        MediaSource {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaSource {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaSource> for Any {
    fn from(s: MediaSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaSource> for Any {
    fn from(s: &MediaSource) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaSource);

impl MediaSource {
    /// The `new MediaSource(..)` constructor, creating a new MediaSource instance
    pub fn new() -> MediaSource {
        Self {
            inner: Any::global("MediaSource").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl MediaSource {
    /// Getter of the `handle` attribute.
    /// [`MediaSource.handle`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/handle)
    pub fn handle(&self) -> MediaSourceHandle {
        self.inner.get("handle").as_::<MediaSourceHandle>()
    }
}
impl MediaSource {
    /// Getter of the `sourceBuffers` attribute.
    /// [`MediaSource.sourceBuffers`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)
    pub fn source_buffers(&self) -> SourceBufferList {
        self.inner.get("sourceBuffers").as_::<SourceBufferList>()
    }
}
impl MediaSource {
    /// Getter of the `activeSourceBuffers` attribute.
    /// [`MediaSource.activeSourceBuffers`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)
    pub fn active_source_buffers(&self) -> SourceBufferList {
        self.inner
            .get("activeSourceBuffers")
            .as_::<SourceBufferList>()
    }
}
impl MediaSource {
    /// Getter of the `readyState` attribute.
    /// [`MediaSource.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)
    pub fn ready_state(&self) -> ReadyState {
        self.inner.get("readyState").as_::<ReadyState>()
    }
}
impl MediaSource {
    /// Getter of the `duration` attribute.
    /// [`MediaSource.duration`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }

    /// Setter of the `duration` attribute.
    /// [`MediaSource.duration`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)
    pub fn set_duration(&mut self, value: f64) {
        self.inner.set("duration", value);
    }
}
impl MediaSource {
    /// Getter of the `onsourceopen` attribute.
    /// [`MediaSource.onsourceopen`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)
    pub fn onsourceopen(&self) -> Any {
        self.inner.get("onsourceopen").as_::<Any>()
    }

    /// Setter of the `onsourceopen` attribute.
    /// [`MediaSource.onsourceopen`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)
    pub fn set_onsourceopen(&mut self, value: &Any) {
        self.inner.set("onsourceopen", value);
    }
}
impl MediaSource {
    /// Getter of the `onsourceended` attribute.
    /// [`MediaSource.onsourceended`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)
    pub fn onsourceended(&self) -> Any {
        self.inner.get("onsourceended").as_::<Any>()
    }

    /// Setter of the `onsourceended` attribute.
    /// [`MediaSource.onsourceended`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)
    pub fn set_onsourceended(&mut self, value: &Any) {
        self.inner.set("onsourceended", value);
    }
}
impl MediaSource {
    /// Getter of the `onsourceclose` attribute.
    /// [`MediaSource.onsourceclose`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)
    pub fn onsourceclose(&self) -> Any {
        self.inner.get("onsourceclose").as_::<Any>()
    }

    /// Setter of the `onsourceclose` attribute.
    /// [`MediaSource.onsourceclose`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)
    pub fn set_onsourceclose(&mut self, value: &Any) {
        self.inner.set("onsourceclose", value);
    }
}
impl MediaSource {
    /// Getter of the `canConstructInDedicatedWorker` static attribute.
    /// [`MediaSource.canConstructInDedicatedWorker`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/canConstructInDedicatedWorker)
    pub fn can_construct_in_dedicated_worker() -> bool {
        Any::global("MediaSource")
            .get("canConstructInDedicatedWorker")
            .as_::<bool>()
    }
}
impl MediaSource {
    /// The addSourceBuffer method.
    /// [`MediaSource.addSourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)
    pub fn add_source_buffer(&self, type_: &DOMString) -> SourceBuffer {
        self.inner
            .call("addSourceBuffer", &[type_.into()])
            .as_::<SourceBuffer>()
    }
}
impl MediaSource {
    /// The removeSourceBuffer method.
    /// [`MediaSource.removeSourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)
    pub fn remove_source_buffer(&self, source_buffer: &SourceBuffer) -> Undefined {
        self.inner
            .call("removeSourceBuffer", &[source_buffer.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    /// The endOfStream method.
    /// [`MediaSource.endOfStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)
    pub fn end_of_stream0(&self) -> Undefined {
        self.inner.call("endOfStream", &[]).as_::<Undefined>()
    }
    /// The endOfStream method.
    /// [`MediaSource.endOfStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)
    pub fn end_of_stream1(&self, error: &EndOfStreamError) -> Undefined {
        self.inner
            .call("endOfStream", &[error.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    /// The setLiveSeekableRange method.
    /// [`MediaSource.setLiveSeekableRange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)
    pub fn set_live_seekable_range(&self, start: f64, end: f64) -> Undefined {
        self.inner
            .call("setLiveSeekableRange", &[start.into(), end.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    /// The clearLiveSeekableRange method.
    /// [`MediaSource.clearLiveSeekableRange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)
    pub fn clear_live_seekable_range(&self) -> Undefined {
        self.inner
            .call("clearLiveSeekableRange", &[])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    /// The isTypeSupported method.
    /// [`MediaSource.isTypeSupported`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported)
    pub fn is_type_supported(type_: &DOMString) -> bool {
        Any::global("MediaSource")
            .call("isTypeSupported", &[type_.into()])
            .as_::<bool>()
    }
}
