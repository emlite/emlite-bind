use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameCallbackMetadata {
    inner: Any,
}
impl FromVal for VideoFrameCallbackMetadata {
    fn from_val(v: &Any) -> Self {
        VideoFrameCallbackMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameCallbackMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameCallbackMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoFrameCallbackMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoFrameCallbackMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoFrameCallbackMetadata> for Any {
    fn from(s: VideoFrameCallbackMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoFrameCallbackMetadata> for Any {
    fn from(s: &VideoFrameCallbackMetadata) -> Any {
        s.inner.clone()
    }
}

impl VideoFrameCallbackMetadata {
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }

    pub fn set_presentation_time(&mut self, value: &Any) {
        self.inner.set("presentationTime", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn expected_display_time(&self) -> Any {
        self.inner.get("expectedDisplayTime").as_::<Any>()
    }

    pub fn set_expected_display_time(&mut self, value: &Any) {
        self.inner.set("expectedDisplayTime", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn media_time(&self) -> f64 {
        self.inner.get("mediaTime").as_::<f64>()
    }

    pub fn set_media_time(&mut self, value: f64) {
        self.inner.set("mediaTime", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn presented_frames(&self) -> u32 {
        self.inner.get("presentedFrames").as_::<u32>()
    }

    pub fn set_presented_frames(&mut self, value: u32) {
        self.inner.set("presentedFrames", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn processing_duration(&self) -> f64 {
        self.inner.get("processingDuration").as_::<f64>()
    }

    pub fn set_processing_duration(&mut self, value: f64) {
        self.inner.set("processingDuration", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn capture_time(&self) -> Any {
        self.inner.get("captureTime").as_::<Any>()
    }

    pub fn set_capture_time(&mut self, value: &Any) {
        self.inner.set("captureTime", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn receive_time(&self) -> Any {
        self.inner.get("receiveTime").as_::<Any>()
    }

    pub fn set_receive_time(&mut self, value: &Any) {
        self.inner.set("receiveTime", value);
    }
}
impl VideoFrameCallbackMetadata {
    pub fn rtp_timestamp(&self) -> u32 {
        self.inner.get("rtpTimestamp").as_::<u32>()
    }

    pub fn set_rtp_timestamp(&mut self, value: u32) {
        self.inner.set("rtpTimestamp", value);
    }
}
