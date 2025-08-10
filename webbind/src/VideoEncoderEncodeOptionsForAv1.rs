use super::*;

/// The VideoEncoderEncodeOptionsForAv1 dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptionsForAv1 {
    inner: Any,
}

impl FromVal for VideoEncoderEncodeOptionsForAv1 {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptionsForAv1 { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderEncodeOptionsForAv1 {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderEncodeOptionsForAv1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderEncodeOptionsForAv1 {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderEncodeOptionsForAv1 {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoEncoderEncodeOptionsForAv1> for Any {
    fn from(s: VideoEncoderEncodeOptionsForAv1) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderEncodeOptionsForAv1> for Any {
    fn from(s: &VideoEncoderEncodeOptionsForAv1) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderEncodeOptionsForAv1 {
    /// Getter of the `quantizer` attribute.
    pub fn quantizer(&self) -> u16 {
        self.inner.get("quantizer").as_::<u16>()
    }

    /// Setter of the `quantizer` attribute.
    pub fn set_quantizer(&mut self, value: u16) {
        self.inner.set("quantizer", value);
    }
}
