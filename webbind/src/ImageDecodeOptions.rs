use super::*;

/// The ImageDecodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecodeOptions {
    inner: Any,
}

impl FromVal for ImageDecodeOptions {
    fn from_val(v: &Any) -> Self {
        ImageDecodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageDecodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageDecodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageDecodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageDecodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ImageDecodeOptions> for Any {
    fn from(s: ImageDecodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageDecodeOptions> for Any {
    fn from(s: &ImageDecodeOptions) -> Any {
        s.inner.clone()
    }
}

impl ImageDecodeOptions {
    /// Getter of the `frameIndex` attribute.
    pub fn frame_index(&self) -> u32 {
        self.inner.get("frameIndex").as_::<u32>()
    }

    /// Setter of the `frameIndex` attribute.
    pub fn set_frame_index(&mut self, value: u32) {
        self.inner.set("frameIndex", value);
    }
}
impl ImageDecodeOptions {
    /// Getter of the `completeFramesOnly` attribute.
    pub fn complete_frames_only(&self) -> bool {
        self.inner.get("completeFramesOnly").as_::<bool>()
    }

    /// Setter of the `completeFramesOnly` attribute.
    pub fn set_complete_frames_only(&mut self, value: bool) {
        self.inner.set("completeFramesOnly", value);
    }
}
