use super::*;

/// The ImageBitmapRenderingContext class.
/// [`ImageBitmapRenderingContext`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageBitmapRenderingContext {
    inner: Any,
}

impl FromVal for ImageBitmapRenderingContext {
    fn from_val(v: &Any) -> Self {
        ImageBitmapRenderingContext {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageBitmapRenderingContext {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageBitmapRenderingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageBitmapRenderingContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageBitmapRenderingContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ImageBitmapRenderingContext> for Any {
    fn from(s: ImageBitmapRenderingContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageBitmapRenderingContext> for Any {
    fn from(s: &ImageBitmapRenderingContext) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageBitmapRenderingContext);

impl ImageBitmapRenderingContext {
    /// Getter of the `canvas` attribute.
    /// [`ImageBitmapRenderingContext.canvas`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/canvas)
    pub fn canvas(&self) -> Any {
        self.inner.get("canvas").as_::<Any>()
    }
}
impl ImageBitmapRenderingContext {
    /// The transferFromImageBitmap method.
    /// [`ImageBitmapRenderingContext.transferFromImageBitmap`](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)
    pub fn transfer_from_image_bitmap(&self, bitmap: &ImageBitmap) -> Undefined {
        self.inner
            .call("transferFromImageBitmap", &[bitmap.into()])
            .as_::<Undefined>()
    }
}
