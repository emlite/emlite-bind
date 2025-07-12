use super::*;

#[derive(Clone, Debug)]
pub struct VideoPlaybackQuality {
    inner: emlite::Val,
}
impl FromVal for VideoPlaybackQuality {
    fn from_val(v: &emlite::Val) -> Self {
        VideoPlaybackQuality {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoPlaybackQuality {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoPlaybackQuality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoPlaybackQuality> for emlite::Val {
    fn from(s: VideoPlaybackQuality) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoPlaybackQuality {
    pub fn creation_time(&self) -> jsbind::Any {
        self.inner.get("creationTime").as_::<jsbind::Any>()
    }
}
impl VideoPlaybackQuality {
    pub fn dropped_video_frames(&self) -> u32 {
        self.inner.get("droppedVideoFrames").as_::<u32>()
    }
}
impl VideoPlaybackQuality {
    pub fn total_video_frames(&self) -> u32 {
        self.inner.get("totalVideoFrames").as_::<u32>()
    }
}
impl VideoPlaybackQuality {
    pub fn corrupted_video_frames(&self) -> u32 {
        self.inner.get("corruptedVideoFrames").as_::<u32>()
    }
}
