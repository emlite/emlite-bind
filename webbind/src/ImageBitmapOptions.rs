use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageBitmapOptions {
    inner: Any,
}
impl FromVal for ImageBitmapOptions {
    fn from_val(v: &Any) -> Self {
        ImageBitmapOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageBitmapOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageBitmapOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ImageBitmapOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ImageBitmapOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ImageBitmapOptions> for Any {
    fn from(s: ImageBitmapOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ImageBitmapOptions> for Any {
    fn from(s: &ImageBitmapOptions) -> Any {
        s.inner.clone()
    }
}

impl ImageBitmapOptions {
    pub fn image_orientation(&self) -> ImageOrientation {
        self.inner.get("imageOrientation").as_::<ImageOrientation>()
    }

    pub fn set_image_orientation(&mut self, value: &ImageOrientation) {
        self.inner.set("imageOrientation", value);
    }
}
impl ImageBitmapOptions {
    pub fn premultiply_alpha(&self) -> PremultiplyAlpha {
        self.inner.get("premultiplyAlpha").as_::<PremultiplyAlpha>()
    }

    pub fn set_premultiply_alpha(&mut self, value: &PremultiplyAlpha) {
        self.inner.set("premultiplyAlpha", value);
    }
}
impl ImageBitmapOptions {
    pub fn color_space_conversion(&self) -> ColorSpaceConversion {
        self.inner
            .get("colorSpaceConversion")
            .as_::<ColorSpaceConversion>()
    }

    pub fn set_color_space_conversion(&mut self, value: &ColorSpaceConversion) {
        self.inner.set("colorSpaceConversion", value);
    }
}
impl ImageBitmapOptions {
    pub fn resize_width(&self) -> u32 {
        self.inner.get("resizeWidth").as_::<u32>()
    }

    pub fn set_resize_width(&mut self, value: u32) {
        self.inner.set("resizeWidth", value);
    }
}
impl ImageBitmapOptions {
    pub fn resize_height(&self) -> u32 {
        self.inner.get("resizeHeight").as_::<u32>()
    }

    pub fn set_resize_height(&mut self, value: u32) {
        self.inner.set("resizeHeight", value);
    }
}
impl ImageBitmapOptions {
    pub fn resize_quality(&self) -> ResizeQuality {
        self.inner.get("resizeQuality").as_::<ResizeQuality>()
    }

    pub fn set_resize_quality(&mut self, value: &ResizeQuality) {
        self.inner.set("resizeQuality", value);
    }
}
