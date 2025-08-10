use super::*;

/// The VideoEncoderEncodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptions {
    inner: Any,
}

impl FromVal for VideoEncoderEncodeOptions {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderEncodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderEncodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderEncodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoEncoderEncodeOptions> for Any {
    fn from(s: VideoEncoderEncodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderEncodeOptions> for Any {
    fn from(s: &VideoEncoderEncodeOptions) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderEncodeOptions {
    /// Getter of the `keyFrame` attribute.
    pub fn key_frame(&self) -> bool {
        self.inner.get("keyFrame").as_::<bool>()
    }

    /// Setter of the `keyFrame` attribute.
    pub fn set_key_frame(&mut self, value: bool) {
        self.inner.set("keyFrame", value);
    }
}
