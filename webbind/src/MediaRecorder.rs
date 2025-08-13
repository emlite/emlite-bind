use super::*;




/// The MediaRecorder class.
/// [`MediaRecorder`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaRecorder {
    inner: EventTarget,
}

impl FromVal for MediaRecorder {
    fn from_val(v: &Any) -> Self {
        MediaRecorder { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaRecorder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaRecorder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaRecorder {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaRecorder> for Any {
    fn from(s: MediaRecorder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaRecorder> for Any {
    fn from(s: &MediaRecorder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaRecorder);



impl MediaRecorder {
    /// The `new MediaRecorder(..)` constructor, creating a new MediaRecorder instance
    pub fn new0(stream: &MediaStream) -> MediaRecorder {
        Self {
            inner: Any::global("MediaRecorder").new(&[stream.into()]).as_::<EventTarget>(),
        }
    }

    /// The `new MediaRecorder(..)` constructor, creating a new MediaRecorder instance
    pub fn new1(stream: &MediaStream, options: &MediaRecorderOptions) -> MediaRecorder {
        Self {
            inner: Any::global("MediaRecorder").new(&[stream.into(), options.into()]).as_::<EventTarget>(),
        }
    }

}
impl MediaRecorder {
    /// Getter of the `stream` attribute.
    /// [`MediaRecorder.stream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)
    pub fn stream(&self) -> MediaStream {
        self.inner.get("stream").as_::<MediaStream>()
    }

}
impl MediaRecorder {
    /// Getter of the `mimeType` attribute.
    /// [`MediaRecorder.mimeType`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)
    pub fn mime_type(&self) -> JsString {
        self.inner.get("mimeType").as_::<JsString>()
    }

}
impl MediaRecorder {
    /// Getter of the `state` attribute.
    /// [`MediaRecorder.state`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)
    pub fn state(&self) -> RecordingState {
        self.inner.get("state").as_::<RecordingState>()
    }

}
impl MediaRecorder {
    /// Getter of the `onstart` attribute.
    /// [`MediaRecorder.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)
    pub fn onstart(&self) -> Any {
        self.inner.get("onstart").as_::<Any>()
    }

    /// Setter of the `onstart` attribute.
    /// [`MediaRecorder.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)
    pub fn set_onstart(&mut self, value: &Any) {
        self.inner.set("onstart", value);
    }
}
impl MediaRecorder {
    /// Getter of the `onstop` attribute.
    /// [`MediaRecorder.onstop`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)
    pub fn onstop(&self) -> Any {
        self.inner.get("onstop").as_::<Any>()
    }

    /// Setter of the `onstop` attribute.
    /// [`MediaRecorder.onstop`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)
    pub fn set_onstop(&mut self, value: &Any) {
        self.inner.set("onstop", value);
    }
}
impl MediaRecorder {
    /// Getter of the `ondataavailable` attribute.
    /// [`MediaRecorder.ondataavailable`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)
    pub fn ondataavailable(&self) -> Any {
        self.inner.get("ondataavailable").as_::<Any>()
    }

    /// Setter of the `ondataavailable` attribute.
    /// [`MediaRecorder.ondataavailable`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)
    pub fn set_ondataavailable(&mut self, value: &Any) {
        self.inner.set("ondataavailable", value);
    }
}
impl MediaRecorder {
    /// Getter of the `onpause` attribute.
    /// [`MediaRecorder.onpause`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onpause)
    pub fn onpause(&self) -> Any {
        self.inner.get("onpause").as_::<Any>()
    }

    /// Setter of the `onpause` attribute.
    /// [`MediaRecorder.onpause`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onpause)
    pub fn set_onpause(&mut self, value: &Any) {
        self.inner.set("onpause", value);
    }
}
impl MediaRecorder {
    /// Getter of the `onresume` attribute.
    /// [`MediaRecorder.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onresume)
    pub fn onresume(&self) -> Any {
        self.inner.get("onresume").as_::<Any>()
    }

    /// Setter of the `onresume` attribute.
    /// [`MediaRecorder.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onresume)
    pub fn set_onresume(&mut self, value: &Any) {
        self.inner.set("onresume", value);
    }
}
impl MediaRecorder {
    /// Getter of the `onerror` attribute.
    /// [`MediaRecorder.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`MediaRecorder.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl MediaRecorder {
    /// Getter of the `videoBitsPerSecond` attribute.
    /// [`MediaRecorder.videoBitsPerSecond`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/videoBitsPerSecond)
    pub fn video_bits_per_second(&self) -> u32 {
        self.inner.get("videoBitsPerSecond").as_::<u32>()
    }

}
impl MediaRecorder {
    /// Getter of the `audioBitsPerSecond` attribute.
    /// [`MediaRecorder.audioBitsPerSecond`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/audioBitsPerSecond)
    pub fn audio_bits_per_second(&self) -> u32 {
        self.inner.get("audioBitsPerSecond").as_::<u32>()
    }

}
impl MediaRecorder {
    /// Getter of the `audioBitrateMode` attribute.
    /// [`MediaRecorder.audioBitrateMode`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/audioBitrateMode)
    pub fn audio_bitrate_mode(&self) -> BitrateMode {
        self.inner.get("audioBitrateMode").as_::<BitrateMode>()
    }

}
impl MediaRecorder {
    /// The start method.
    /// [`MediaRecorder.start`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)
    pub fn start0(&self, ) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
    /// The start method.
    /// [`MediaRecorder.start`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)
    pub fn start1(&self, timeslice: u32) -> Undefined {
        self.inner.call("start", &[timeslice.into(), ]).as_::<Undefined>()
    }
}
impl MediaRecorder {
    /// The stop method.
    /// [`MediaRecorder.stop`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)
    pub fn stop(&self, ) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl MediaRecorder {
    /// The pause method.
    /// [`MediaRecorder.pause`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)
    pub fn pause(&self, ) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }
}
impl MediaRecorder {
    /// The resume method.
    /// [`MediaRecorder.resume`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)
    pub fn resume(&self, ) -> Undefined {
        self.inner.call("resume", &[]).as_::<Undefined>()
    }
}
impl MediaRecorder {
    /// The requestData method.
    /// [`MediaRecorder.requestData`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)
    pub fn request_data(&self, ) -> Undefined {
        self.inner.call("requestData", &[]).as_::<Undefined>()
    }
}
impl MediaRecorder {
    /// The isTypeSupported method.
    /// [`MediaRecorder.isTypeSupported`](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported)
    pub fn is_type_supported(type_: &JsString) -> bool {
        Any::global("MediaRecorder").call("isTypeSupported", &[type_.into(), ]).as_::<bool>()
    }
}
