use super::*;




/// The VideoDecoderSupport dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecoderSupport {
    inner: Any,
}

impl FromVal for VideoDecoderSupport {
    fn from_val(v: &Any) -> Self {
        VideoDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoDecoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoDecoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoDecoderSupport {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoDecoderSupport> for Any {
    fn from(s: VideoDecoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoDecoderSupport> for Any {
    fn from(s: &VideoDecoderSupport) -> Any {
        s.inner.clone()
    }
}

impl VideoDecoderSupport {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl VideoDecoderSupport {
    /// Getter of the `config` attribute.
    pub fn config(&self) -> VideoDecoderConfig {
        self.inner.get("config").as_::<VideoDecoderConfig>()
    }

    /// Setter of the `config` attribute.
    pub fn set_config(&mut self, value: &VideoDecoderConfig) {
        self.inner.set("config", value);
    }
}
