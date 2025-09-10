use super::*;

/// The VideoEncoderEncodeOptionsForVp9 dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptionsForVp9 {
    inner: Any,
}

impl FromVal for VideoEncoderEncodeOptionsForVp9 {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptionsForVp9 { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderEncodeOptionsForVp9 {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderEncodeOptionsForVp9 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderEncodeOptionsForVp9 {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderEncodeOptionsForVp9 {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoEncoderEncodeOptionsForVp9> for Any {
    fn from(s: VideoEncoderEncodeOptionsForVp9) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderEncodeOptionsForVp9> for Any {
    fn from(s: &VideoEncoderEncodeOptionsForVp9) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderEncodeOptionsForVp9 {
    /// Getter of the `quantizer` attribute.
    pub fn quantizer(&self) -> u16 {
        self.inner.get("quantizer").as_::<u16>()
    }

    /// Setter of the `quantizer` attribute.
    pub fn set_quantizer(&mut self, value: u16) {
        self.inner.set("quantizer", value);
    }
}
