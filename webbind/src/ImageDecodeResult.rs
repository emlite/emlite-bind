use super::*;




/// The ImageDecodeResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecodeResult {
    inner: Any,
}

impl FromVal for ImageDecodeResult {
    fn from_val(v: &Any) -> Self {
        ImageDecodeResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageDecodeResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageDecodeResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageDecodeResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageDecodeResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageDecodeResult> for Any {
    fn from(s: ImageDecodeResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageDecodeResult> for Any {
    fn from(s: &ImageDecodeResult) -> Any {
        s.inner.clone()
    }
}

impl ImageDecodeResult {
    /// Getter of the `image` attribute.
    pub fn image(&self) -> VideoFrame {
        self.inner.get("image").as_::<VideoFrame>()
    }

    /// Setter of the `image` attribute.
    pub fn set_image(&mut self, value: &VideoFrame) {
        self.inner.set("image", value);
    }
}
impl ImageDecodeResult {
    /// Getter of the `complete` attribute.
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }

    /// Setter of the `complete` attribute.
    pub fn set_complete(&mut self, value: bool) {
        self.inner.set("complete", value);
    }
}
