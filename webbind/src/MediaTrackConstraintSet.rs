use super::*;

/// The MediaTrackConstraintSet dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackConstraintSet {
    inner: Any,
}

impl FromVal for MediaTrackConstraintSet {
    fn from_val(v: &Any) -> Self {
        MediaTrackConstraintSet { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaTrackConstraintSet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaTrackConstraintSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaTrackConstraintSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaTrackConstraintSet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaTrackConstraintSet> for Any {
    fn from(s: MediaTrackConstraintSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaTrackConstraintSet> for Any {
    fn from(s: &MediaTrackConstraintSet) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackConstraintSet {
    /// Getter of the `displaySurface` attribute.
    pub fn display_surface(&self) -> Any {
        self.inner.get("displaySurface").as_::<Any>()
    }

    /// Setter of the `displaySurface` attribute.
    pub fn set_display_surface(&mut self, value: &Any) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackConstraintSet {
    /// Getter of the `logicalSurface` attribute.
    pub fn logical_surface(&self) -> Any {
        self.inner.get("logicalSurface").as_::<Any>()
    }

    /// Setter of the `logicalSurface` attribute.
    pub fn set_logical_surface(&mut self, value: &Any) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackConstraintSet {
    /// Getter of the `cursor` attribute.
    pub fn cursor(&self) -> Any {
        self.inner.get("cursor").as_::<Any>()
    }

    /// Setter of the `cursor` attribute.
    pub fn set_cursor(&mut self, value: &Any) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackConstraintSet {
    /// Getter of the `restrictOwnAudio` attribute.
    pub fn restrict_own_audio(&self) -> Any {
        self.inner.get("restrictOwnAudio").as_::<Any>()
    }

    /// Setter of the `restrictOwnAudio` attribute.
    pub fn set_restrict_own_audio(&mut self, value: &Any) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackConstraintSet {
    /// Getter of the `suppressLocalAudioPlayback` attribute.
    pub fn suppress_local_audio_playback(&self) -> Any {
        self.inner.get("suppressLocalAudioPlayback").as_::<Any>()
    }

    /// Setter of the `suppressLocalAudioPlayback` attribute.
    pub fn set_suppress_local_audio_playback(&mut self, value: &Any) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
