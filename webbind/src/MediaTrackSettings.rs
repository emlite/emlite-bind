use super::*;




/// The MediaTrackSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackSettings {
    inner: Any,
}

impl FromVal for MediaTrackSettings {
    fn from_val(v: &Any) -> Self {
        MediaTrackSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaTrackSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaTrackSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaTrackSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaTrackSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaTrackSettings> for Any {
    fn from(s: MediaTrackSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaTrackSettings> for Any {
    fn from(s: &MediaTrackSettings) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackSettings {
    /// Getter of the `displaySurface` attribute.
    pub fn display_surface(&self) -> JsString {
        self.inner.get("displaySurface").as_::<JsString>()
    }

    /// Setter of the `displaySurface` attribute.
    pub fn set_display_surface(&mut self, value: &JsString) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackSettings {
    /// Getter of the `logicalSurface` attribute.
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    /// Setter of the `logicalSurface` attribute.
    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackSettings {
    /// Getter of the `cursor` attribute.
    pub fn cursor(&self) -> JsString {
        self.inner.get("cursor").as_::<JsString>()
    }

    /// Setter of the `cursor` attribute.
    pub fn set_cursor(&mut self, value: &JsString) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackSettings {
    /// Getter of the `restrictOwnAudio` attribute.
    pub fn restrict_own_audio(&self) -> bool {
        self.inner.get("restrictOwnAudio").as_::<bool>()
    }

    /// Setter of the `restrictOwnAudio` attribute.
    pub fn set_restrict_own_audio(&mut self, value: bool) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackSettings {
    /// Getter of the `suppressLocalAudioPlayback` attribute.
    pub fn suppress_local_audio_playback(&self) -> bool {
        self.inner.get("suppressLocalAudioPlayback").as_::<bool>()
    }

    /// Setter of the `suppressLocalAudioPlayback` attribute.
    pub fn set_suppress_local_audio_playback(&mut self, value: bool) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
impl MediaTrackSettings {
    /// Getter of the `screenPixelRatio` attribute.
    pub fn screen_pixel_ratio(&self) -> f64 {
        self.inner.get("screenPixelRatio").as_::<f64>()
    }

    /// Setter of the `screenPixelRatio` attribute.
    pub fn set_screen_pixel_ratio(&mut self, value: f64) {
        self.inner.set("screenPixelRatio", value);
    }
}
