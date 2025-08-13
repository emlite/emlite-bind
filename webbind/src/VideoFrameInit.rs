use super::*;




/// The VideoFrameInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameInit {
    inner: Any,
}

impl FromVal for VideoFrameInit {
    fn from_val(v: &Any) -> Self {
        VideoFrameInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoFrameInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoFrameInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoFrameInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoFrameInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoFrameInit> for Any {
    fn from(s: VideoFrameInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoFrameInit> for Any {
    fn from(s: &VideoFrameInit) -> Any {
        s.inner.clone()
    }
}

impl VideoFrameInit {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> AlphaOption {
        self.inner.get("alpha").as_::<AlphaOption>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: &AlphaOption) {
        self.inner.set("alpha", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `visibleRect` attribute.
    pub fn visible_rect(&self) -> DOMRectInit {
        self.inner.get("visibleRect").as_::<DOMRectInit>()
    }

    /// Setter of the `visibleRect` attribute.
    pub fn set_visible_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("visibleRect", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `rotation` attribute.
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }

    /// Setter of the `rotation` attribute.
    pub fn set_rotation(&mut self, value: f64) {
        self.inner.set("rotation", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `flip` attribute.
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }

    /// Setter of the `flip` attribute.
    pub fn set_flip(&mut self, value: bool) {
        self.inner.set("flip", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `displayWidth` attribute.
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }

    /// Setter of the `displayWidth` attribute.
    pub fn set_display_width(&mut self, value: u32) {
        self.inner.set("displayWidth", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `displayHeight` attribute.
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }

    /// Setter of the `displayHeight` attribute.
    pub fn set_display_height(&mut self, value: u32) {
        self.inner.set("displayHeight", value);
    }
}
impl VideoFrameInit {
    /// Getter of the `metadata` attribute.
    pub fn metadata(&self) -> VideoFrameMetadata {
        self.inner.get("metadata").as_::<VideoFrameMetadata>()
    }

    /// Setter of the `metadata` attribute.
    pub fn set_metadata(&mut self, value: &VideoFrameMetadata) {
        self.inner.set("metadata", value);
    }
}
