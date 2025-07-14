use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoTrackGenerator {
    inner: emlite::Val,
}
impl FromVal for VideoTrackGenerator {
    fn from_val(v: &emlite::Val) -> Self {
        VideoTrackGenerator {
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
impl core::ops::Deref for VideoTrackGenerator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoTrackGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoTrackGenerator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoTrackGenerator {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoTrackGenerator> for emlite::Val {
    fn from(s: VideoTrackGenerator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VideoTrackGenerator);

impl VideoTrackGenerator {
    pub fn new() -> VideoTrackGenerator {
        Self {
            inner: emlite::Val::global("VideoTrackGenerator")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl VideoTrackGenerator {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl VideoTrackGenerator {
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }

    pub fn set_muted(&mut self, value: bool) {
        self.inner.set("muted", value);
    }
}
impl VideoTrackGenerator {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
