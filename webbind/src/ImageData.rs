use super::*;




/// The ImageData class.
/// [`ImageData`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageData {
    inner: Any,
}

impl FromVal for ImageData {
    fn from_val(v: &Any) -> Self {
        ImageData { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageData> for Any {
    fn from(s: ImageData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageData> for Any {
    fn from(s: &ImageData) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageData);



impl ImageData {
    /// The `new ImageData(..)` constructor, creating a new ImageData instance
    pub fn new0(data: &Any, sw: u32) -> ImageData {
        Self {
            inner: Any::global("ImageData").new(&[data.into(), sw.into()]).as_::<Any>(),
        }
    }

    /// The `new ImageData(..)` constructor, creating a new ImageData instance
    pub fn new1(data: &Any, sw: u32, sh: u32) -> ImageData {
        Self {
            inner: Any::global("ImageData").new(&[data.into(), sw.into(), sh.into()]).as_::<Any>(),
        }
    }

    /// The `new ImageData(..)` constructor, creating a new ImageData instance
    pub fn new2(data: &Any, sw: u32, sh: u32, settings: &ImageDataSettings) -> ImageData {
        Self {
            inner: Any::global("ImageData").new(&[data.into(), sw.into(), sh.into(), settings.into()]).as_::<Any>(),
        }
    }

}
impl ImageData {
    /// Getter of the `width` attribute.
    /// [`ImageData.width`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

}
impl ImageData {
    /// Getter of the `height` attribute.
    /// [`ImageData.height`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

}
impl ImageData {
    /// Getter of the `data` attribute.
    /// [`ImageData.data`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

}
impl ImageData {
    /// Getter of the `pixelFormat` attribute.
    /// [`ImageData.pixelFormat`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/pixelFormat)
    pub fn pixel_format(&self) -> ImageDataPixelFormat {
        self.inner.get("pixelFormat").as_::<ImageDataPixelFormat>()
    }

}
impl ImageData {
    /// Getter of the `colorSpace` attribute.
    /// [`ImageData.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/colorSpace)
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

}
