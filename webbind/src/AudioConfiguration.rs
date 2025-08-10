use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioConfiguration {
    inner: Any,
}
impl FromVal for AudioConfiguration {
    fn from_val(v: &Any) -> Self {
        AudioConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioConfiguration> for Any {
    fn from(s: AudioConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioConfiguration> for Any {
    fn from(s: &AudioConfiguration) -> Any {
        s.inner.clone()
    }
}

impl AudioConfiguration {
    pub fn content_type(&self) -> JsString {
        self.inner.get("contentType").as_::<JsString>()
    }

    pub fn set_content_type(&mut self, value: &JsString) {
        self.inner.set("contentType", value);
    }
}
impl AudioConfiguration {
    pub fn channels(&self) -> JsString {
        self.inner.get("channels").as_::<JsString>()
    }

    pub fn set_channels(&mut self, value: &JsString) {
        self.inner.set("channels", value);
    }
}
impl AudioConfiguration {
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl AudioConfiguration {
    pub fn samplerate(&self) -> u32 {
        self.inner.get("samplerate").as_::<u32>()
    }

    pub fn set_samplerate(&mut self, value: u32) {
        self.inner.set("samplerate", value);
    }
}
impl AudioConfiguration {
    pub fn spatial_rendering(&self) -> bool {
        self.inner.get("spatialRendering").as_::<bool>()
    }

    pub fn set_spatial_rendering(&mut self, value: bool) {
        self.inner.set("spatialRendering", value);
    }
}
