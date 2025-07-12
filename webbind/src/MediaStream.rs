use super::*;

#[derive(Clone, Debug)]
pub struct MediaStream {
    inner: EventTarget,
}
impl FromVal for MediaStream {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStream {
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
impl std::ops::Deref for MediaStream {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaStream> for emlite::Val {
    fn from(s: MediaStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStream {
    pub fn new(tracks: jsbind::Sequence<MediaStreamTrack>) -> MediaStream {
        Self {
            inner: emlite::Val::global("MediaStream")
                .new(&[tracks.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl MediaStream {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl MediaStream {
    pub fn get_audio_tracks(&self) -> jsbind::Sequence<MediaStreamTrack> {
        self.inner
            .call("getAudioTracks", &[])
            .as_::<jsbind::Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_video_tracks(&self) -> jsbind::Sequence<MediaStreamTrack> {
        self.inner
            .call("getVideoTracks", &[])
            .as_::<jsbind::Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_tracks(&self) -> jsbind::Sequence<MediaStreamTrack> {
        self.inner
            .call("getTracks", &[])
            .as_::<jsbind::Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_track_by_id(&self, track_id: jsbind::DOMString) -> MediaStreamTrack {
        self.inner
            .call("getTrackById", &[track_id.into()])
            .as_::<MediaStreamTrack>()
    }
}
impl MediaStream {
    pub fn add_track(&self, track: MediaStreamTrack) -> jsbind::Undefined {
        self.inner
            .call("addTrack", &[track.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaStream {
    pub fn remove_track(&self, track: MediaStreamTrack) -> jsbind::Undefined {
        self.inner
            .call("removeTrack", &[track.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MediaStream {
    pub fn clone_(&self) -> MediaStream {
        self.inner.call("clone", &[]).as_::<MediaStream>()
    }
}
impl MediaStream {
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }
}
impl MediaStream {
    pub fn onaddtrack(&self) -> jsbind::Any {
        self.inner.get("onaddtrack").as_::<jsbind::Any>()
    }

    pub fn set_onaddtrack(&mut self, value: jsbind::Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl MediaStream {
    pub fn onremovetrack(&self) -> jsbind::Any {
        self.inner.get("onremovetrack").as_::<jsbind::Any>()
    }

    pub fn set_onremovetrack(&mut self, value: jsbind::Any) {
        self.inner.set("onremovetrack", value);
    }
}
