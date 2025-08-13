use super::*;




/// The VideoPlaybackQuality class.
/// [`VideoPlaybackQuality`](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoPlaybackQuality {
    inner: Any,
}

impl FromVal for VideoPlaybackQuality {
    fn from_val(v: &Any) -> Self {
        VideoPlaybackQuality { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoPlaybackQuality {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoPlaybackQuality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoPlaybackQuality {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoPlaybackQuality {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoPlaybackQuality> for Any {
    fn from(s: VideoPlaybackQuality) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoPlaybackQuality> for Any {
    fn from(s: &VideoPlaybackQuality) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VideoPlaybackQuality);


impl VideoPlaybackQuality {
    /// Getter of the `creationTime` attribute.
    /// [`VideoPlaybackQuality.creationTime`](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)
    pub fn creation_time(&self) -> Any {
        self.inner.get("creationTime").as_::<Any>()
    }

}
impl VideoPlaybackQuality {
    /// Getter of the `droppedVideoFrames` attribute.
    /// [`VideoPlaybackQuality.droppedVideoFrames`](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)
    pub fn dropped_video_frames(&self) -> u32 {
        self.inner.get("droppedVideoFrames").as_::<u32>()
    }

}
impl VideoPlaybackQuality {
    /// Getter of the `totalVideoFrames` attribute.
    /// [`VideoPlaybackQuality.totalVideoFrames`](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)
    pub fn total_video_frames(&self) -> u32 {
        self.inner.get("totalVideoFrames").as_::<u32>()
    }

}
impl VideoPlaybackQuality {
    /// Getter of the `corruptedVideoFrames` attribute.
    /// [`VideoPlaybackQuality.corruptedVideoFrames`](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)
    pub fn corrupted_video_frames(&self) -> u32 {
        self.inner.get("corruptedVideoFrames").as_::<u32>()
    }

}
