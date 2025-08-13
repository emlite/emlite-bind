use super::*;




/// The MediaTrackSupportedConstraints dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackSupportedConstraints {
    inner: Any,
}

impl FromVal for MediaTrackSupportedConstraints {
    fn from_val(v: &Any) -> Self {
        MediaTrackSupportedConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaTrackSupportedConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaTrackSupportedConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaTrackSupportedConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaTrackSupportedConstraints {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaTrackSupportedConstraints> for Any {
    fn from(s: MediaTrackSupportedConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaTrackSupportedConstraints> for Any {
    fn from(s: &MediaTrackSupportedConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackSupportedConstraints {
    /// Getter of the `displaySurface` attribute.
    pub fn display_surface(&self) -> bool {
        self.inner.get("displaySurface").as_::<bool>()
    }

    /// Setter of the `displaySurface` attribute.
    pub fn set_display_surface(&mut self, value: bool) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    /// Getter of the `logicalSurface` attribute.
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    /// Setter of the `logicalSurface` attribute.
    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    /// Getter of the `cursor` attribute.
    pub fn cursor(&self) -> bool {
        self.inner.get("cursor").as_::<bool>()
    }

    /// Setter of the `cursor` attribute.
    pub fn set_cursor(&mut self, value: bool) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackSupportedConstraints {
    /// Getter of the `restrictOwnAudio` attribute.
    pub fn restrict_own_audio(&self) -> bool {
        self.inner.get("restrictOwnAudio").as_::<bool>()
    }

    /// Setter of the `restrictOwnAudio` attribute.
    pub fn set_restrict_own_audio(&mut self, value: bool) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackSupportedConstraints {
    /// Getter of the `suppressLocalAudioPlayback` attribute.
    pub fn suppress_local_audio_playback(&self) -> bool {
        self.inner.get("suppressLocalAudioPlayback").as_::<bool>()
    }

    /// Setter of the `suppressLocalAudioPlayback` attribute.
    pub fn set_suppress_local_audio_playback(&mut self, value: bool) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
