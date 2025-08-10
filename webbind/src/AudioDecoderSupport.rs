use super::*;

/// The AudioDecoderSupport dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderSupport {
    inner: Any,
}

impl FromVal for AudioDecoderSupport {
    fn from_val(v: &Any) -> Self {
        AudioDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDecoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDecoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDecoderSupport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDecoderSupport> for Any {
    fn from(s: AudioDecoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDecoderSupport> for Any {
    fn from(s: &AudioDecoderSupport) -> Any {
        s.inner.clone()
    }
}

impl AudioDecoderSupport {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AudioDecoderSupport {
    /// Getter of the `config` attribute.
    pub fn config(&self) -> AudioDecoderConfig {
        self.inner.get("config").as_::<AudioDecoderConfig>()
    }

    /// Setter of the `config` attribute.
    pub fn set_config(&mut self, value: &AudioDecoderConfig) {
        self.inner.set("config", value);
    }
}
