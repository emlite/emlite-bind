use super::*;

/// The AudioDataCopyToOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDataCopyToOptions {
    inner: Any,
}

impl FromVal for AudioDataCopyToOptions {
    fn from_val(v: &Any) -> Self {
        AudioDataCopyToOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDataCopyToOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDataCopyToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDataCopyToOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDataCopyToOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDataCopyToOptions> for Any {
    fn from(s: AudioDataCopyToOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDataCopyToOptions> for Any {
    fn from(s: &AudioDataCopyToOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioDataCopyToOptions {
    /// Getter of the `planeIndex` attribute.
    pub fn plane_index(&self) -> u32 {
        self.inner.get("planeIndex").as_::<u32>()
    }

    /// Setter of the `planeIndex` attribute.
    pub fn set_plane_index(&mut self, value: u32) {
        self.inner.set("planeIndex", value);
    }
}
impl AudioDataCopyToOptions {
    /// Getter of the `frameOffset` attribute.
    pub fn frame_offset(&self) -> u32 {
        self.inner.get("frameOffset").as_::<u32>()
    }

    /// Setter of the `frameOffset` attribute.
    pub fn set_frame_offset(&mut self, value: u32) {
        self.inner.set("frameOffset", value);
    }
}
impl AudioDataCopyToOptions {
    /// Getter of the `frameCount` attribute.
    pub fn frame_count(&self) -> u32 {
        self.inner.get("frameCount").as_::<u32>()
    }

    /// Setter of the `frameCount` attribute.
    pub fn set_frame_count(&mut self, value: u32) {
        self.inner.set("frameCount", value);
    }
}
impl AudioDataCopyToOptions {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> AudioSampleFormat {
        self.inner.get("format").as_::<AudioSampleFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &AudioSampleFormat) {
        self.inner.set("format", value);
    }
}
