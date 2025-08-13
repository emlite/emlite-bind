use super::*;




/// The SourceBuffer class.
/// [`SourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SourceBuffer {
    inner: EventTarget,
}

impl FromVal for SourceBuffer {
    fn from_val(v: &Any) -> Self {
        SourceBuffer { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SourceBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SourceBuffer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SourceBuffer> for Any {
    fn from(s: SourceBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SourceBuffer> for Any {
    fn from(s: &SourceBuffer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SourceBuffer);


impl SourceBuffer {
    /// Getter of the `mode` attribute.
    /// [`SourceBuffer.mode`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)
    pub fn mode(&self) -> AppendMode {
        self.inner.get("mode").as_::<AppendMode>()
    }

    /// Setter of the `mode` attribute.
    /// [`SourceBuffer.mode`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)
    pub fn set_mode(&mut self, value: &AppendMode) {
        self.inner.set("mode", value);
    }
}
impl SourceBuffer {
    /// Getter of the `updating` attribute.
    /// [`SourceBuffer.updating`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)
    pub fn updating(&self) -> bool {
        self.inner.get("updating").as_::<bool>()
    }

}
impl SourceBuffer {
    /// Getter of the `buffered` attribute.
    /// [`SourceBuffer.buffered`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)
    pub fn buffered(&self) -> TimeRanges {
        self.inner.get("buffered").as_::<TimeRanges>()
    }

}
impl SourceBuffer {
    /// Getter of the `timestampOffset` attribute.
    /// [`SourceBuffer.timestampOffset`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)
    pub fn timestamp_offset(&self) -> f64 {
        self.inner.get("timestampOffset").as_::<f64>()
    }

    /// Setter of the `timestampOffset` attribute.
    /// [`SourceBuffer.timestampOffset`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)
    pub fn set_timestamp_offset(&mut self, value: f64) {
        self.inner.set("timestampOffset", value);
    }
}
impl SourceBuffer {
    /// Getter of the `audioTracks` attribute.
    /// [`SourceBuffer.audioTracks`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/audioTracks)
    pub fn audio_tracks(&self) -> AudioTrackList {
        self.inner.get("audioTracks").as_::<AudioTrackList>()
    }

}
impl SourceBuffer {
    /// Getter of the `videoTracks` attribute.
    /// [`SourceBuffer.videoTracks`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/videoTracks)
    pub fn video_tracks(&self) -> VideoTrackList {
        self.inner.get("videoTracks").as_::<VideoTrackList>()
    }

}
impl SourceBuffer {
    /// Getter of the `textTracks` attribute.
    /// [`SourceBuffer.textTracks`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/textTracks)
    pub fn text_tracks(&self) -> TextTrackList {
        self.inner.get("textTracks").as_::<TextTrackList>()
    }

}
impl SourceBuffer {
    /// Getter of the `appendWindowStart` attribute.
    /// [`SourceBuffer.appendWindowStart`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)
    pub fn append_window_start(&self) -> f64 {
        self.inner.get("appendWindowStart").as_::<f64>()
    }

    /// Setter of the `appendWindowStart` attribute.
    /// [`SourceBuffer.appendWindowStart`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)
    pub fn set_append_window_start(&mut self, value: f64) {
        self.inner.set("appendWindowStart", value);
    }
}
impl SourceBuffer {
    /// Getter of the `appendWindowEnd` attribute.
    /// [`SourceBuffer.appendWindowEnd`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)
    pub fn append_window_end(&self) -> f64 {
        self.inner.get("appendWindowEnd").as_::<f64>()
    }

    /// Setter of the `appendWindowEnd` attribute.
    /// [`SourceBuffer.appendWindowEnd`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)
    pub fn set_append_window_end(&mut self, value: f64) {
        self.inner.set("appendWindowEnd", value);
    }
}
impl SourceBuffer {
    /// Getter of the `onupdatestart` attribute.
    /// [`SourceBuffer.onupdatestart`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)
    pub fn onupdatestart(&self) -> Any {
        self.inner.get("onupdatestart").as_::<Any>()
    }

    /// Setter of the `onupdatestart` attribute.
    /// [`SourceBuffer.onupdatestart`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)
    pub fn set_onupdatestart(&mut self, value: &Any) {
        self.inner.set("onupdatestart", value);
    }
}
impl SourceBuffer {
    /// Getter of the `onupdate` attribute.
    /// [`SourceBuffer.onupdate`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)
    pub fn onupdate(&self) -> Any {
        self.inner.get("onupdate").as_::<Any>()
    }

    /// Setter of the `onupdate` attribute.
    /// [`SourceBuffer.onupdate`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)
    pub fn set_onupdate(&mut self, value: &Any) {
        self.inner.set("onupdate", value);
    }
}
impl SourceBuffer {
    /// Getter of the `onupdateend` attribute.
    /// [`SourceBuffer.onupdateend`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)
    pub fn onupdateend(&self) -> Any {
        self.inner.get("onupdateend").as_::<Any>()
    }

    /// Setter of the `onupdateend` attribute.
    /// [`SourceBuffer.onupdateend`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)
    pub fn set_onupdateend(&mut self, value: &Any) {
        self.inner.set("onupdateend", value);
    }
}
impl SourceBuffer {
    /// Getter of the `onerror` attribute.
    /// [`SourceBuffer.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`SourceBuffer.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl SourceBuffer {
    /// Getter of the `onabort` attribute.
    /// [`SourceBuffer.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`SourceBuffer.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl SourceBuffer {
    /// The appendBuffer method.
    /// [`SourceBuffer.appendBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)
    pub fn append_buffer(&self, data: &Any) -> Undefined {
        self.inner.call("appendBuffer", &[data.into(), ]).as_::<Undefined>()
    }
}
impl SourceBuffer {
    /// The abort method.
    /// [`SourceBuffer.abort`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)
    pub fn abort(&self, ) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl SourceBuffer {
    /// The changeType method.
    /// [`SourceBuffer.changeType`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)
    pub fn change_type(&self, type_: &JsString) -> Undefined {
        self.inner.call("changeType", &[type_.into(), ]).as_::<Undefined>()
    }
}
impl SourceBuffer {
    /// The remove method.
    /// [`SourceBuffer.remove`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)
    pub fn remove(&self, start: f64, end: f64) -> Undefined {
        self.inner.call("remove", &[start.into(), end.into(), ]).as_::<Undefined>()
    }
}
