use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameBufferInit {
    inner: Any,
}
impl FromVal for VideoFrameBufferInit {
    fn from_val(v: &Any) -> Self {
        VideoFrameBufferInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameBufferInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameBufferInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoFrameBufferInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoFrameBufferInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoFrameBufferInit> for Any {
    fn from(s: VideoFrameBufferInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoFrameBufferInit> for Any {
    fn from(s: &VideoFrameBufferInit) -> Any {
        s.inner.clone()
    }
}

impl VideoFrameBufferInit {
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }

    pub fn set_format(&mut self, value: &VideoPixelFormat) {
        self.inner.set("format", value);
    }
}
impl VideoFrameBufferInit {
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }

    pub fn set_coded_width(&mut self, value: u32) {
        self.inner.set("codedWidth", value);
    }
}
impl VideoFrameBufferInit {
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }

    pub fn set_coded_height(&mut self, value: u32) {
        self.inner.set("codedHeight", value);
    }
}
impl VideoFrameBufferInit {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl VideoFrameBufferInit {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl VideoFrameBufferInit {
    pub fn layout(&self) -> TypedArray<PlaneLayout> {
        self.inner.get("layout").as_::<TypedArray<PlaneLayout>>()
    }

    pub fn set_layout(&mut self, value: &TypedArray<PlaneLayout>) {
        self.inner.set("layout", value);
    }
}
impl VideoFrameBufferInit {
    pub fn visible_rect(&self) -> DOMRectInit {
        self.inner.get("visibleRect").as_::<DOMRectInit>()
    }

    pub fn set_visible_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("visibleRect", value);
    }
}
impl VideoFrameBufferInit {
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }

    pub fn set_rotation(&mut self, value: f64) {
        self.inner.set("rotation", value);
    }
}
impl VideoFrameBufferInit {
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }

    pub fn set_flip(&mut self, value: bool) {
        self.inner.set("flip", value);
    }
}
impl VideoFrameBufferInit {
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }

    pub fn set_display_width(&mut self, value: u32) {
        self.inner.set("displayWidth", value);
    }
}
impl VideoFrameBufferInit {
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }

    pub fn set_display_height(&mut self, value: u32) {
        self.inner.set("displayHeight", value);
    }
}
impl VideoFrameBufferInit {
    pub fn color_space(&self) -> VideoColorSpaceInit {
        self.inner.get("colorSpace").as_::<VideoColorSpaceInit>()
    }

    pub fn set_color_space(&mut self, value: &VideoColorSpaceInit) {
        self.inner.set("colorSpace", value);
    }
}
impl VideoFrameBufferInit {
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}
impl VideoFrameBufferInit {
    pub fn metadata(&self) -> VideoFrameMetadata {
        self.inner.get("metadata").as_::<VideoFrameMetadata>()
    }

    pub fn set_metadata(&mut self, value: &VideoFrameMetadata) {
        self.inner.set("metadata", value);
    }
}
