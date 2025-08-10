use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FlacEncoderConfig {
    inner: Any,
}
impl FromVal for FlacEncoderConfig {
    fn from_val(v: &Any) -> Self {
        FlacEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FlacEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FlacEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FlacEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FlacEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FlacEncoderConfig> for Any {
    fn from(s: FlacEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FlacEncoderConfig> for Any {
    fn from(s: &FlacEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl FlacEncoderConfig {
    pub fn block_size(&self) -> u32 {
        self.inner.get("blockSize").as_::<u32>()
    }

    pub fn set_block_size(&mut self, value: u32) {
        self.inner.set("blockSize", value);
    }
}
impl FlacEncoderConfig {
    pub fn compress_level(&self) -> u32 {
        self.inner.get("compressLevel").as_::<u32>()
    }

    pub fn set_compress_level(&mut self, value: u32) {
        self.inner.set("compressLevel", value);
    }
}
