use super::*;




/// The VideoFrameBufferInit dictionary.
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
    /// Getter of the `format` attribute.
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &VideoPixelFormat) {
        self.inner.set("format", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `codedWidth` attribute.
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }

    /// Setter of the `codedWidth` attribute.
    pub fn set_coded_width(&mut self, value: u32) {
        self.inner.set("codedWidth", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `codedHeight` attribute.
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }

    /// Setter of the `codedHeight` attribute.
    pub fn set_coded_height(&mut self, value: u32) {
        self.inner.set("codedHeight", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> TypedArray<PlaneLayout> {
        self.inner.get("layout").as_::<TypedArray<PlaneLayout>>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &TypedArray<PlaneLayout>) {
        self.inner.set("layout", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `visibleRect` attribute.
    pub fn visible_rect(&self) -> DOMRectInit {
        self.inner.get("visibleRect").as_::<DOMRectInit>()
    }

    /// Setter of the `visibleRect` attribute.
    pub fn set_visible_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("visibleRect", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `rotation` attribute.
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }

    /// Setter of the `rotation` attribute.
    pub fn set_rotation(&mut self, value: f64) {
        self.inner.set("rotation", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `flip` attribute.
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }

    /// Setter of the `flip` attribute.
    pub fn set_flip(&mut self, value: bool) {
        self.inner.set("flip", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `displayWidth` attribute.
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }

    /// Setter of the `displayWidth` attribute.
    pub fn set_display_width(&mut self, value: u32) {
        self.inner.set("displayWidth", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `displayHeight` attribute.
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }

    /// Setter of the `displayHeight` attribute.
    pub fn set_display_height(&mut self, value: u32) {
        self.inner.set("displayHeight", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> VideoColorSpaceInit {
        self.inner.get("colorSpace").as_::<VideoColorSpaceInit>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &VideoColorSpaceInit) {
        self.inner.set("colorSpace", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}
impl VideoFrameBufferInit {
    /// Getter of the `metadata` attribute.
    pub fn metadata(&self) -> VideoFrameMetadata {
        self.inner.get("metadata").as_::<VideoFrameMetadata>()
    }

    /// Setter of the `metadata` attribute.
    pub fn set_metadata(&mut self, value: &VideoFrameMetadata) {
        self.inner.set("metadata", value);
    }
}
