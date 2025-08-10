use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptionsForHevc {
    inner: Any,
}
impl FromVal for VideoEncoderEncodeOptionsForHevc {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptionsForHevc { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderEncodeOptionsForHevc {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderEncodeOptionsForHevc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoEncoderEncodeOptionsForHevc {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoEncoderEncodeOptionsForHevc {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoEncoderEncodeOptionsForHevc> for Any {
    fn from(s: VideoEncoderEncodeOptionsForHevc) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoEncoderEncodeOptionsForHevc> for Any {
    fn from(s: &VideoEncoderEncodeOptionsForHevc) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderEncodeOptionsForHevc {
    pub fn quantizer(&self) -> u16 {
        self.inner.get("quantizer").as_::<u16>()
    }

    pub fn set_quantizer(&mut self, value: u16) {
        self.inner.set("quantizer", value);
    }
}
