use super::*;




/// The ImageEncodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageEncodeOptions {
    inner: Any,
}

impl FromVal for ImageEncodeOptions {
    fn from_val(v: &Any) -> Self {
        ImageEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageEncodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageEncodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageEncodeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageEncodeOptions> for Any {
    fn from(s: ImageEncodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageEncodeOptions> for Any {
    fn from(s: &ImageEncodeOptions) -> Any {
        s.inner.clone()
    }
}

impl ImageEncodeOptions {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl ImageEncodeOptions {
    /// Getter of the `quality` attribute.
    pub fn quality(&self) -> f64 {
        self.inner.get("quality").as_::<f64>()
    }

    /// Setter of the `quality` attribute.
    pub fn set_quality(&mut self, value: f64) {
        self.inner.set("quality", value);
    }
}
