use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SourceBuffer {
    inner: EventTarget,
}
impl FromVal for SourceBuffer {
    fn from_val(v: &emlite::Val) -> Self {
        SourceBuffer {
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
impl core::ops::Deref for SourceBuffer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SourceBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SourceBuffer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SourceBuffer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SourceBuffer> for emlite::Val {
    fn from(s: SourceBuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SourceBuffer);

impl SourceBuffer {
    pub fn mode(&self) -> AppendMode {
        self.inner.get("mode").as_::<AppendMode>()
    }

    pub fn set_mode(&mut self, value: AppendMode) {
        self.inner.set("mode", value);
    }
}
impl SourceBuffer {
    pub fn updating(&self) -> bool {
        self.inner.get("updating").as_::<bool>()
    }
}
impl SourceBuffer {
    pub fn buffered(&self) -> TimeRanges {
        self.inner.get("buffered").as_::<TimeRanges>()
    }
}
impl SourceBuffer {
    pub fn timestamp_offset(&self) -> f64 {
        self.inner.get("timestampOffset").as_::<f64>()
    }

    pub fn set_timestamp_offset(&mut self, value: f64) {
        self.inner.set("timestampOffset", value);
    }
}
impl SourceBuffer {
    pub fn audio_tracks(&self) -> AudioTrackList {
        self.inner.get("audioTracks").as_::<AudioTrackList>()
    }
}
impl SourceBuffer {
    pub fn video_tracks(&self) -> VideoTrackList {
        self.inner.get("videoTracks").as_::<VideoTrackList>()
    }
}
impl SourceBuffer {
    pub fn text_tracks(&self) -> TextTrackList {
        self.inner.get("textTracks").as_::<TextTrackList>()
    }
}
impl SourceBuffer {
    pub fn append_window_start(&self) -> f64 {
        self.inner.get("appendWindowStart").as_::<f64>()
    }

    pub fn set_append_window_start(&mut self, value: f64) {
        self.inner.set("appendWindowStart", value);
    }
}
impl SourceBuffer {
    pub fn append_window_end(&self) -> f64 {
        self.inner.get("appendWindowEnd").as_::<f64>()
    }

    pub fn set_append_window_end(&mut self, value: f64) {
        self.inner.set("appendWindowEnd", value);
    }
}
impl SourceBuffer {
    pub fn onupdatestart(&self) -> jsbind::Any {
        self.inner.get("onupdatestart").as_::<jsbind::Any>()
    }

    pub fn set_onupdatestart(&mut self, value: jsbind::Any) {
        self.inner.set("onupdatestart", value);
    }
}
impl SourceBuffer {
    pub fn onupdate(&self) -> jsbind::Any {
        self.inner.get("onupdate").as_::<jsbind::Any>()
    }

    pub fn set_onupdate(&mut self, value: jsbind::Any) {
        self.inner.set("onupdate", value);
    }
}
impl SourceBuffer {
    pub fn onupdateend(&self) -> jsbind::Any {
        self.inner.get("onupdateend").as_::<jsbind::Any>()
    }

    pub fn set_onupdateend(&mut self, value: jsbind::Any) {
        self.inner.set("onupdateend", value);
    }
}
impl SourceBuffer {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl SourceBuffer {
    pub fn onabort(&self) -> jsbind::Any {
        self.inner.get("onabort").as_::<jsbind::Any>()
    }

    pub fn set_onabort(&mut self, value: jsbind::Any) {
        self.inner.set("onabort", value);
    }
}
impl SourceBuffer {
    pub fn append_buffer(&self, data: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("appendBuffer", &[data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SourceBuffer {
    pub fn abort(&self) -> jsbind::Undefined {
        self.inner.call("abort", &[]).as_::<jsbind::Undefined>()
    }
}
impl SourceBuffer {
    pub fn change_type(&self, type_: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("changeType", &[type_.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SourceBuffer {
    pub fn remove(&self, start: f64, end: f64) -> jsbind::Undefined {
        self.inner
            .call("remove", &[start.into(), end.into()])
            .as_::<jsbind::Undefined>()
    }
}
