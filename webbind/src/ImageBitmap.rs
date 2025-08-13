use super::*;




/// The ImageBitmap class.
/// [`ImageBitmap`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageBitmap {
    inner: Any,
}

impl FromVal for ImageBitmap {
    fn from_val(v: &Any) -> Self {
        ImageBitmap { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageBitmap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageBitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageBitmap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageBitmap {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageBitmap> for Any {
    fn from(s: ImageBitmap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageBitmap> for Any {
    fn from(s: &ImageBitmap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageBitmap);


impl ImageBitmap {
    /// Getter of the `width` attribute.
    /// [`ImageBitmap.width`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

}
impl ImageBitmap {
    /// Getter of the `height` attribute.
    /// [`ImageBitmap.height`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

}
impl ImageBitmap {
    /// The close method.
    /// [`ImageBitmap.close`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
