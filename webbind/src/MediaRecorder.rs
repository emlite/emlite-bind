use super::*;

#[derive(Clone, Debug)]
pub struct MediaRecorder {
    inner: EventTarget,
}
impl FromVal for MediaRecorder {
    fn from_val(v: &emlite::Val) -> Self {
        MediaRecorder {
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
impl std::ops::Deref for MediaRecorder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaRecorder> for emlite::Val {
    fn from(s: MediaRecorder) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaRecorder {
    pub fn new0(stream: MediaStream) -> MediaRecorder {
        Self {
            inner: emlite::Val::global("MediaRecorder")
                .new(&[stream.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(stream: MediaStream, options: jsbind::Any) -> MediaRecorder {
        Self {
            inner: emlite::Val::global("MediaRecorder")
                .new(&[stream.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl MediaRecorder {
    pub fn stream(&self) -> MediaStream {
        self.inner.get("stream").as_::<MediaStream>()
    }
}
impl MediaRecorder {
    pub fn mime_type(&self) -> jsbind::DOMString {
        self.inner.get("mimeType").as_::<jsbind::DOMString>()
    }
}
impl MediaRecorder {
    pub fn state(&self) -> RecordingState {
        self.inner.get("state").as_::<RecordingState>()
    }
}
impl MediaRecorder {
    pub fn onstart(&self) -> jsbind::Any {
        self.inner.get("onstart").as_::<jsbind::Any>()
    }

    pub fn set_onstart(&mut self, value: jsbind::Any) {
        self.inner.set("onstart", value);
    }
}
impl MediaRecorder {
    pub fn onstop(&self) -> jsbind::Any {
        self.inner.get("onstop").as_::<jsbind::Any>()
    }

    pub fn set_onstop(&mut self, value: jsbind::Any) {
        self.inner.set("onstop", value);
    }
}
impl MediaRecorder {
    pub fn ondataavailable(&self) -> jsbind::Any {
        self.inner.get("ondataavailable").as_::<jsbind::Any>()
    }

    pub fn set_ondataavailable(&mut self, value: jsbind::Any) {
        self.inner.set("ondataavailable", value);
    }
}
impl MediaRecorder {
    pub fn onpause(&self) -> jsbind::Any {
        self.inner.get("onpause").as_::<jsbind::Any>()
    }

    pub fn set_onpause(&mut self, value: jsbind::Any) {
        self.inner.set("onpause", value);
    }
}
impl MediaRecorder {
    pub fn onresume(&self) -> jsbind::Any {
        self.inner.get("onresume").as_::<jsbind::Any>()
    }

    pub fn set_onresume(&mut self, value: jsbind::Any) {
        self.inner.set("onresume", value);
    }
}
impl MediaRecorder {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl MediaRecorder {
    pub fn video_bits_per_second(&self) -> u32 {
        self.inner.get("videoBitsPerSecond").as_::<u32>()
    }
}
impl MediaRecorder {
    pub fn audio_bits_per_second(&self) -> u32 {
        self.inner.get("audioBitsPerSecond").as_::<u32>()
    }
}
impl MediaRecorder {
    pub fn audio_bitrate_mode(&self) -> BitrateMode {
        self.inner.get("audioBitrateMode").as_::<BitrateMode>()
    }
}
impl MediaRecorder {
    pub fn start0(&self) -> jsbind::Undefined {
        self.inner.call("start", &[]).as_::<jsbind::Undefined>()
    }

    pub fn start1(&self, timeslice: u32) -> jsbind::Undefined {
        self.inner
            .call("start", &[timeslice.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaRecorder {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl MediaRecorder {
    pub fn pause(&self) -> jsbind::Undefined {
        self.inner.call("pause", &[]).as_::<jsbind::Undefined>()
    }
}
impl MediaRecorder {
    pub fn resume(&self) -> jsbind::Undefined {
        self.inner.call("resume", &[]).as_::<jsbind::Undefined>()
    }
}
impl MediaRecorder {
    pub fn request_data(&self) -> jsbind::Undefined {
        self.inner
            .call("requestData", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaRecorder {
    pub fn is_type_supported(type_: jsbind::DOMString) -> bool {
        emlite::Val::global("mediarecorder")
            .call("isTypeSupported", &[type_.into()])
            .as_::<bool>()
    }
}
