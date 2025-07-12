use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for VideoTrackList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoTrackList> for emlite::Val {
    fn from(s: VideoTrackList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoTrackList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl VideoTrackList {
    pub fn get_track_by_id(&self, id: jsbind::DOMString) -> VideoTrack {
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
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
impl VideoTrackList {
    pub fn onaddtrack(&self) -> jsbind::Any {
        self.inner.get("onaddtrack").as_::<jsbind::Any>()
    }

    pub fn set_onaddtrack(&mut self, value: jsbind::Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl VideoTrackList {
    pub fn onremovetrack(&self) -> jsbind::Any {
        self.inner.get("onremovetrack").as_::<jsbind::Any>()
    }

    pub fn set_onremovetrack(&mut self, value: jsbind::Any) {
        self.inner.set("onremovetrack", value);
    }
}
