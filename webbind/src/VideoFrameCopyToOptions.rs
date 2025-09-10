use super::*;

/// The VideoFrameCopyToOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameCopyToOptions {
    inner: Any,
}

impl FromVal for VideoFrameCopyToOptions {
    fn from_val(v: &Any) -> Self {
        VideoFrameCopyToOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoFrameCopyToOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoFrameCopyToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoFrameCopyToOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoFrameCopyToOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoFrameCopyToOptions> for Any {
    fn from(s: VideoFrameCopyToOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoFrameCopyToOptions> for Any {
    fn from(s: &VideoFrameCopyToOptions) -> Any {
        s.inner.clone()
    }
}

impl VideoFrameCopyToOptions {
    /// Getter of the `rect` attribute.
    pub fn rect(&self) -> DOMRectInit {
        self.inner.get("rect").as_::<DOMRectInit>()
    }

    /// Setter of the `rect` attribute.
    pub fn set_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("rect", value);
    }
}
impl VideoFrameCopyToOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> TypedArray<PlaneLayout> {
        self.inner.get("layout").as_::<TypedArray<PlaneLayout>>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &TypedArray<PlaneLayout>) {
        self.inner.set("layout", value);
    }
}
impl VideoFrameCopyToOptions {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &VideoPixelFormat) {
        self.inner.set("format", value);
    }
}
impl VideoFrameCopyToOptions {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
