use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoTrackList {
    inner: EventTarget,
}
impl FromVal for VideoTrackList {
    fn from_val(v: &emlite::Val) -> Self {
        VideoTrackList {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoTrackList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoTrackList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoTrackList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoTrackList> for emlite::Val {
    fn from(s: VideoTrackList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VideoTrackList);

impl VideoTrackList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl VideoTrackList {
    pub fn get_track_by_id(&self, id: DOMString) -> VideoTrack {
        self.inner
            .call("getTrackById", &[id.into()])
            .as_::<VideoTrack>()
    }
}
impl VideoTrackList {
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }
}
impl VideoTrackList {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }
}
impl VideoTrackList {
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    pub fn set_onaddtrack(&mut self, value: Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl VideoTrackList {
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    pub fn set_onremovetrack(&mut self, value: Any) {
        self.inner.set("onremovetrack", value);
    }
}
