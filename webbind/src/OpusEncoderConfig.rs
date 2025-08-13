use super::*;




/// The OpusEncoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OpusEncoderConfig {
    inner: Any,
}

impl FromVal for OpusEncoderConfig {
    fn from_val(v: &Any) -> Self {
        OpusEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OpusEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OpusEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OpusEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OpusEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OpusEncoderConfig> for Any {
    fn from(s: OpusEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OpusEncoderConfig> for Any {
    fn from(s: &OpusEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl OpusEncoderConfig {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> OpusBitstreamFormat {
        self.inner.get("format").as_::<OpusBitstreamFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &OpusBitstreamFormat) {
        self.inner.set("format", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> OpusSignal {
        self.inner.get("signal").as_::<OpusSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &OpusSignal) {
        self.inner.set("signal", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `application` attribute.
    pub fn application(&self) -> OpusApplication {
        self.inner.get("application").as_::<OpusApplication>()
    }

    /// Setter of the `application` attribute.
    pub fn set_application(&mut self, value: &OpusApplication) {
        self.inner.set("application", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `frameDuration` attribute.
    pub fn frame_duration(&self) -> u64 {
        self.inner.get("frameDuration").as_::<u64>()
    }

    /// Setter of the `frameDuration` attribute.
    pub fn set_frame_duration(&mut self, value: u64) {
        self.inner.set("frameDuration", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `complexity` attribute.
    pub fn complexity(&self) -> u32 {
        self.inner.get("complexity").as_::<u32>()
    }

    /// Setter of the `complexity` attribute.
    pub fn set_complexity(&mut self, value: u32) {
        self.inner.set("complexity", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `packetlossperc` attribute.
    pub fn packetlossperc(&self) -> u32 {
        self.inner.get("packetlossperc").as_::<u32>()
    }

    /// Setter of the `packetlossperc` attribute.
    pub fn set_packetlossperc(&mut self, value: u32) {
        self.inner.set("packetlossperc", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `useinbandfec` attribute.
    pub fn useinbandfec(&self) -> bool {
        self.inner.get("useinbandfec").as_::<bool>()
    }

    /// Setter of the `useinbandfec` attribute.
    pub fn set_useinbandfec(&mut self, value: bool) {
        self.inner.set("useinbandfec", value);
    }
}
impl OpusEncoderConfig {
    /// Getter of the `usedtx` attribute.
    pub fn usedtx(&self) -> bool {
        self.inner.get("usedtx").as_::<bool>()
    }

    /// Setter of the `usedtx` attribute.
    pub fn set_usedtx(&mut self, value: bool) {
        self.inner.set("usedtx", value);
    }
}
