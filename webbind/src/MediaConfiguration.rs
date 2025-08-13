use super::*;




/// The MediaConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaConfiguration {
    inner: Any,
}

impl FromVal for MediaConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaConfiguration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaConfiguration> for Any {
    fn from(s: MediaConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaConfiguration> for Any {
    fn from(s: &MediaConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaConfiguration {
    /// Getter of the `video` attribute.
    pub fn video(&self) -> VideoConfiguration {
        self.inner.get("video").as_::<VideoConfiguration>()
    }

    /// Setter of the `video` attribute.
    pub fn set_video(&mut self, value: &VideoConfiguration) {
        self.inner.set("video", value);
    }
}
impl MediaConfiguration {
    /// Getter of the `audio` attribute.
    pub fn audio(&self) -> AudioConfiguration {
        self.inner.get("audio").as_::<AudioConfiguration>()
    }

    /// Setter of the `audio` attribute.
    pub fn set_audio(&mut self, value: &AudioConfiguration) {
        self.inner.set("audio", value);
    }
}
