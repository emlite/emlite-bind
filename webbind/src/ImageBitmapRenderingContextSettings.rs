use super::*;




/// The ImageBitmapRenderingContextSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageBitmapRenderingContextSettings {
    inner: Any,
}

impl FromVal for ImageBitmapRenderingContextSettings {
    fn from_val(v: &Any) -> Self {
        ImageBitmapRenderingContextSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageBitmapRenderingContextSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageBitmapRenderingContextSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageBitmapRenderingContextSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageBitmapRenderingContextSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageBitmapRenderingContextSettings> for Any {
    fn from(s: ImageBitmapRenderingContextSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageBitmapRenderingContextSettings> for Any {
    fn from(s: &ImageBitmapRenderingContextSettings) -> Any {
        s.inner.clone()
    }
}

impl ImageBitmapRenderingContextSettings {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
