use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaSource {
    inner: EventTarget,
}
impl FromVal for MediaSource {
    fn from_val(v: &emlite::Val) -> Self {
        MediaSource {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for MediaSource {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaSource {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaSource> for emlite::Val {
    fn from(s: MediaSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaSource);

impl MediaSource {
    pub fn new() -> MediaSource {
        Self {
            inner: emlite::Val::global("MediaSource")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl MediaSource {
    pub fn handle(&self) -> MediaSourceHandle {
        self.inner.get("handle").as_::<MediaSourceHandle>()
    }
}
impl MediaSource {
    pub fn source_buffers(&self) -> SourceBufferList {
        self.inner.get("sourceBuffers").as_::<SourceBufferList>()
    }
}
impl MediaSource {
    pub fn active_source_buffers(&self) -> SourceBufferList {
        self.inner
            .get("activeSourceBuffers")
            .as_::<SourceBufferList>()
    }
}
impl MediaSource {
    pub fn ready_state(&self) -> ReadyState {
        self.inner.get("readyState").as_::<ReadyState>()
    }
}
impl MediaSource {
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }

    pub fn set_duration(&mut self, value: f64) {
        self.inner.set("duration", value);
    }
}
impl MediaSource {
    pub fn onsourceopen(&self) -> Any {
        self.inner.get("onsourceopen").as_::<Any>()
    }

    pub fn set_onsourceopen(&mut self, value: Any) {
        self.inner.set("onsourceopen", value);
    }
}
impl MediaSource {
    pub fn onsourceended(&self) -> Any {
        self.inner.get("onsourceended").as_::<Any>()
    }

    pub fn set_onsourceended(&mut self, value: Any) {
        self.inner.set("onsourceended", value);
    }
}
impl MediaSource {
    pub fn onsourceclose(&self) -> Any {
        self.inner.get("onsourceclose").as_::<Any>()
    }

    pub fn set_onsourceclose(&mut self, value: Any) {
        self.inner.set("onsourceclose", value);
    }
}
impl MediaSource {
    pub fn can_construct_in_dedicated_worker() -> bool {
        emlite::Val::global("MediaSource")
            .get("canConstructInDedicatedWorker")
            .as_::<bool>()
    }
}
impl MediaSource {
    pub fn add_source_buffer(&self, type_: DOMString) -> SourceBuffer {
        self.inner
            .call("addSourceBuffer", &[type_.into()])
            .as_::<SourceBuffer>()
    }
}
impl MediaSource {
    pub fn remove_source_buffer(&self, source_buffer: SourceBuffer) -> Undefined {
        self.inner
            .call("removeSourceBuffer", &[source_buffer.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    pub fn end_of_stream0(&self) -> Undefined {
        self.inner.call("endOfStream", &[]).as_::<Undefined>()
    }

    pub fn end_of_stream1(&self, error: EndOfStreamError) -> Undefined {
        self.inner
            .call("endOfStream", &[error.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    pub fn set_live_seekable_range(&self, start: f64, end: f64) -> Undefined {
        self.inner
            .call("setLiveSeekableRange", &[start.into(), end.into()])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    pub fn clear_live_seekable_range(&self) -> Undefined {
        self.inner
            .call("clearLiveSeekableRange", &[])
            .as_::<Undefined>()
    }
}
impl MediaSource {
    pub fn is_type_supported(type_: DOMString) -> bool {
        emlite::Val::global("MediaSource")
            .call("isTypeSupported", &[type_.into()])
            .as_::<bool>()
    }
}
