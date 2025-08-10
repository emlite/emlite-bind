use super::*;

/// The MediaElementAudioSourceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaElementAudioSourceOptions {
    inner: Any,
}

impl FromVal for MediaElementAudioSourceOptions {
    fn from_val(v: &Any) -> Self {
        MediaElementAudioSourceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaElementAudioSourceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaElementAudioSourceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaElementAudioSourceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaElementAudioSourceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaElementAudioSourceOptions> for Any {
    fn from(s: MediaElementAudioSourceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaElementAudioSourceOptions> for Any {
    fn from(s: &MediaElementAudioSourceOptions) -> Any {
        s.inner.clone()
    }
}

impl MediaElementAudioSourceOptions {
    /// Getter of the `mediaElement` attribute.
    pub fn media_element(&self) -> HTMLMediaElement {
        self.inner.get("mediaElement").as_::<HTMLMediaElement>()
    }

    /// Setter of the `mediaElement` attribute.
    pub fn set_media_element(&mut self, value: &HTMLMediaElement) {
        self.inner.set("mediaElement", value);
    }
}
