use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageTrackList {
    inner: emlite::Val,
}
impl FromVal for ImageTrackList {
    fn from_val(v: &emlite::Val) -> Self {
        ImageTrackList {
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
impl core::ops::Deref for ImageTrackList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageTrackList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageTrackList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageTrackList> for emlite::Val {
    fn from(s: ImageTrackList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ImageTrackList);

impl ImageTrackList {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl ImageTrackList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl ImageTrackList {
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }
}
impl ImageTrackList {
    pub fn selected_track(&self) -> ImageTrack {
        self.inner.get("selectedTrack").as_::<ImageTrack>()
    }
}
