use super::*;

/// The VideoEncoderSupport dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderSupport {
    inner: Any,
}

impl FromVal for VideoEncoderSupport {
    fn from_val(v: &Any) -> Self {
        VideoEncoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderSupport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoEncoderSupport> for Any {
    fn from(s: VideoEncoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderSupport> for Any {
    fn from(s: &VideoEncoderSupport) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderSupport {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl VideoEncoderSupport {
    /// Getter of the `config` attribute.
    pub fn config(&self) -> VideoEncoderConfig {
        self.inner.get("config").as_::<VideoEncoderConfig>()
    }

    /// Setter of the `config` attribute.
    pub fn set_config(&mut self, value: &VideoEncoderConfig) {
        self.inner.set("config", value);
    }
}
