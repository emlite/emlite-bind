use super::*;

/// The MediaStreamTrackAudioSourceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackAudioSourceOptions {
    inner: Any,
}

impl FromVal for MediaStreamTrackAudioSourceOptions {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackAudioSourceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaStreamTrackAudioSourceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamTrackAudioSourceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamTrackAudioSourceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamTrackAudioSourceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaStreamTrackAudioSourceOptions> for Any {
    fn from(s: MediaStreamTrackAudioSourceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamTrackAudioSourceOptions> for Any {
    fn from(s: &MediaStreamTrackAudioSourceOptions) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamTrackAudioSourceOptions {
    /// Getter of the `mediaStreamTrack` attribute.
    pub fn media_stream_track(&self) -> MediaStreamTrack {
        self.inner.get("mediaStreamTrack").as_::<MediaStreamTrack>()
    }

    /// Setter of the `mediaStreamTrack` attribute.
    pub fn set_media_stream_track(&mut self, value: &MediaStreamTrack) {
        self.inner.set("mediaStreamTrack", value);
    }
}
