use super::*;




/// The ImageDataSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDataSettings {
    inner: Any,
}

impl FromVal for ImageDataSettings {
    fn from_val(v: &Any) -> Self {
        ImageDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageDataSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageDataSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageDataSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageDataSettings> for Any {
    fn from(s: ImageDataSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageDataSettings> for Any {
    fn from(s: &ImageDataSettings) -> Any {
        s.inner.clone()
    }
}

impl ImageDataSettings {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl ImageDataSettings {
    /// Getter of the `pixelFormat` attribute.
    pub fn pixel_format(&self) -> ImageDataPixelFormat {
        self.inner.get("pixelFormat").as_::<ImageDataPixelFormat>()
    }

    /// Setter of the `pixelFormat` attribute.
    pub fn set_pixel_format(&mut self, value: &ImageDataPixelFormat) {
        self.inner.set("pixelFormat", value);
    }
}
