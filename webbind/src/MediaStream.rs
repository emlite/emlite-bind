use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MediaStream {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaStream> for emlite::Val {
    fn from(s: MediaStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaStream> for emlite::Val {
    fn from(s: &MediaStream) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaStream);

impl MediaStream {
    pub fn new(tracks: Sequence<MediaStreamTrack>) -> MediaStream {
        Self {
            inner: emlite::Val::global("MediaStream")
                .new(&[tracks.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl MediaStream {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl MediaStream {
    pub fn get_audio_tracks(&self) -> Sequence<MediaStreamTrack> {
        self.inner
            .call("getAudioTracks", &[])
            .as_::<Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_video_tracks(&self) -> Sequence<MediaStreamTrack> {
        self.inner
            .call("getVideoTracks", &[])
            .as_::<Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_tracks(&self) -> Sequence<MediaStreamTrack> {
        self.inner
            .call("getTracks", &[])
            .as_::<Sequence<MediaStreamTrack>>()
    }
}
impl MediaStream {
    pub fn get_track_by_id(&self, track_id: DOMString) -> MediaStreamTrack {
        self.inner
            .call("getTrackById", &[track_id.into()])
            .as_::<MediaStreamTrack>()
    }
}
impl MediaStream {
    pub fn add_track(&self, track: MediaStreamTrack) -> Undefined {
        self.inner
            .call("addTrack", &[track.into()])
            .as_::<Undefined>()
    }
}
impl MediaStream {
    pub fn remove_track(&self, track: MediaStreamTrack) -> Undefined {
        self.inner
            .call("removeTrack", &[track.into()])
            .as_::<Undefined>()
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
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    pub fn set_onaddtrack(&mut self, value: Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl MediaStream {
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    pub fn set_onremovetrack(&mut self, value: Any) {
        self.inner.set("onremovetrack", value);
    }
}
