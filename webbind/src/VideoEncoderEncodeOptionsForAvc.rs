use super::*;




/// The VideoEncoderEncodeOptionsForAvc dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptionsForAvc {
    inner: Any,
}

impl FromVal for VideoEncoderEncodeOptionsForAvc {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptionsForAvc { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderEncodeOptionsForAvc {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderEncodeOptionsForAvc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderEncodeOptionsForAvc {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderEncodeOptionsForAvc {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoEncoderEncodeOptionsForAvc> for Any {
    fn from(s: VideoEncoderEncodeOptionsForAvc) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderEncodeOptionsForAvc> for Any {
    fn from(s: &VideoEncoderEncodeOptionsForAvc) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderEncodeOptionsForAvc {
    /// Getter of the `quantizer` attribute.
    pub fn quantizer(&self) -> u16 {
        self.inner.get("quantizer").as_::<u16>()
    }

    /// Setter of the `quantizer` attribute.
    pub fn set_quantizer(&mut self, value: u16) {
        self.inner.set("quantizer", value);
    }
}
