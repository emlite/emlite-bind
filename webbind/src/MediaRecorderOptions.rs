use super::*;




/// The MediaRecorderOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaRecorderOptions {
    inner: Any,
}

impl FromVal for MediaRecorderOptions {
    fn from_val(v: &Any) -> Self {
        MediaRecorderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaRecorderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaRecorderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaRecorderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaRecorderOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaRecorderOptions> for Any {
    fn from(s: MediaRecorderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaRecorderOptions> for Any {
    fn from(s: &MediaRecorderOptions) -> Any {
        s.inner.clone()
    }
}

impl MediaRecorderOptions {
    /// Getter of the `mimeType` attribute.
    pub fn mime_type(&self) -> JsString {
        self.inner.get("mimeType").as_::<JsString>()
    }

    /// Setter of the `mimeType` attribute.
    pub fn set_mime_type(&mut self, value: &JsString) {
        self.inner.set("mimeType", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `audioBitsPerSecond` attribute.
    pub fn audio_bits_per_second(&self) -> u32 {
        self.inner.get("audioBitsPerSecond").as_::<u32>()
    }

    /// Setter of the `audioBitsPerSecond` attribute.
    pub fn set_audio_bits_per_second(&mut self, value: u32) {
        self.inner.set("audioBitsPerSecond", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `videoBitsPerSecond` attribute.
    pub fn video_bits_per_second(&self) -> u32 {
        self.inner.get("videoBitsPerSecond").as_::<u32>()
    }

    /// Setter of the `videoBitsPerSecond` attribute.
    pub fn set_video_bits_per_second(&mut self, value: u32) {
        self.inner.set("videoBitsPerSecond", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `bitsPerSecond` attribute.
    pub fn bits_per_second(&self) -> u32 {
        self.inner.get("bitsPerSecond").as_::<u32>()
    }

    /// Setter of the `bitsPerSecond` attribute.
    pub fn set_bits_per_second(&mut self, value: u32) {
        self.inner.set("bitsPerSecond", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `audioBitrateMode` attribute.
    pub fn audio_bitrate_mode(&self) -> BitrateMode {
        self.inner.get("audioBitrateMode").as_::<BitrateMode>()
    }

    /// Setter of the `audioBitrateMode` attribute.
    pub fn set_audio_bitrate_mode(&mut self, value: &BitrateMode) {
        self.inner.set("audioBitrateMode", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `videoKeyFrameIntervalDuration` attribute.
    pub fn video_key_frame_interval_duration(&self) -> Any {
        self.inner.get("videoKeyFrameIntervalDuration").as_::<Any>()
    }

    /// Setter of the `videoKeyFrameIntervalDuration` attribute.
    pub fn set_video_key_frame_interval_duration(&mut self, value: &Any) {
        self.inner.set("videoKeyFrameIntervalDuration", value);
    }
}
impl MediaRecorderOptions {
    /// Getter of the `videoKeyFrameIntervalCount` attribute.
    pub fn video_key_frame_interval_count(&self) -> u32 {
        self.inner.get("videoKeyFrameIntervalCount").as_::<u32>()
    }

    /// Setter of the `videoKeyFrameIntervalCount` attribute.
    pub fn set_video_key_frame_interval_count(&mut self, value: u32) {
        self.inner.set("videoKeyFrameIntervalCount", value);
    }
}
