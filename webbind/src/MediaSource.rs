use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MediaSource {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaSource> for emlite::Val {
    fn from(s: MediaSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn onsourceopen(&self) -> jsbind::Any {
        self.inner.get("onsourceopen").as_::<jsbind::Any>()
    }

    pub fn set_onsourceopen(&mut self, value: jsbind::Any) {
        self.inner.set("onsourceopen", value);
    }
}
impl MediaSource {
    pub fn onsourceended(&self) -> jsbind::Any {
        self.inner.get("onsourceended").as_::<jsbind::Any>()
    }

    pub fn set_onsourceended(&mut self, value: jsbind::Any) {
        self.inner.set("onsourceended", value);
    }
}
impl MediaSource {
    pub fn onsourceclose(&self) -> jsbind::Any {
        self.inner.get("onsourceclose").as_::<jsbind::Any>()
    }

    pub fn set_onsourceclose(&mut self, value: jsbind::Any) {
        self.inner.set("onsourceclose", value);
    }
}
impl MediaSource {
    pub fn can_construct_in_dedicated_worker() -> bool {
        emlite::Val::global("mediasource")
            .get("canConstructInDedicatedWorker")
            .as_::<bool>()
    }
}
impl MediaSource {
    pub fn add_source_buffer(&self, type_: jsbind::DOMString) -> SourceBuffer {
        self.inner
            .call("addSourceBuffer", &[type_.into()])
            .as_::<SourceBuffer>()
    }
}
impl MediaSource {
    pub fn remove_source_buffer(&self, source_buffer: SourceBuffer) -> jsbind::Undefined {
        self.inner
            .call("removeSourceBuffer", &[source_buffer.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaSource {
    pub fn end_of_stream0(&self) -> jsbind::Undefined {
        self.inner
            .call("endOfStream", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn end_of_stream1(&self, error: EndOfStreamError) -> jsbind::Undefined {
        self.inner
            .call("endOfStream", &[error.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaSource {
    pub fn set_live_seekable_range(&self, start: f64, end: f64) -> jsbind::Undefined {
        self.inner
            .call("setLiveSeekableRange", &[start.into(), end.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaSource {
    pub fn clear_live_seekable_range(&self) -> jsbind::Undefined {
        self.inner
            .call("clearLiveSeekableRange", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaSource {
    pub fn is_type_supported(type_: jsbind::DOMString) -> bool {
        emlite::Val::global("mediasource")
            .call("isTypeSupported", &[type_.into()])
            .as_::<bool>()
    }
}
