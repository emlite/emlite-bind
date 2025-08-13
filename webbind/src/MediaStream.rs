use super::*;




/// The MediaStream class.
/// [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStream {
    inner: EventTarget,
}

impl FromVal for MediaStream {
    fn from_val(v: &Any) -> Self {
        MediaStream { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for MediaStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStream {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaStream> for Any {
    fn from(s: MediaStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStream> for Any {
    fn from(s: &MediaStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaStream);



impl MediaStream {
    /// The `new MediaStream(..)` constructor, creating a new MediaStream instance
    pub fn new(tracks: &TypedArray<MediaStreamTrack>) -> MediaStream {
        Self {
            inner: Any::global("MediaStream").new(&[tracks.into()]).as_::<EventTarget>(),
        }
    }

}
impl MediaStream {
    /// Getter of the `id` attribute.
    /// [`MediaStream.id`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

}
impl MediaStream {
    /// The getAudioTracks method.
    /// [`MediaStream.getAudioTracks`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getAudioTracks)
    pub fn get_audio_tracks(&self, ) -> TypedArray<MediaStreamTrack> {
        self.inner.call("getAudioTracks", &[]).as_::<TypedArray<MediaStreamTrack>>()
    }
}
impl MediaStream {
    /// The getVideoTracks method.
    /// [`MediaStream.getVideoTracks`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getVideoTracks)
    pub fn get_video_tracks(&self, ) -> TypedArray<MediaStreamTrack> {
        self.inner.call("getVideoTracks", &[]).as_::<TypedArray<MediaStreamTrack>>()
    }
}
impl MediaStream {
    /// The getTracks method.
    /// [`MediaStream.getTracks`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTracks)
    pub fn get_tracks(&self, ) -> TypedArray<MediaStreamTrack> {
        self.inner.call("getTracks", &[]).as_::<TypedArray<MediaStreamTrack>>()
    }
}
impl MediaStream {
    /// The getTrackById method.
    /// [`MediaStream.getTrackById`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTrackById)
    pub fn get_track_by_id(&self, track_id: &JsString) -> MediaStreamTrack {
        self.inner.call("getTrackById", &[track_id.into(), ]).as_::<MediaStreamTrack>()
    }
}
impl MediaStream {
    /// The addTrack method.
    /// [`MediaStream.addTrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/addTrack)
    pub fn add_track(&self, track: &MediaStreamTrack) -> Undefined {
        self.inner.call("addTrack", &[track.into(), ]).as_::<Undefined>()
    }
}
impl MediaStream {
    /// The removeTrack method.
    /// [`MediaStream.removeTrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/removeTrack)
    pub fn remove_track(&self, track: &MediaStreamTrack) -> Undefined {
        self.inner.call("removeTrack", &[track.into(), ]).as_::<Undefined>()
    }
}
impl MediaStream {
    /// The clone method.
    /// [`MediaStream.clone`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/clone)
    pub fn clone_(&self, ) -> MediaStream {
        self.inner.call("clone", &[]).as_::<MediaStream>()
    }
}
impl MediaStream {
    /// Getter of the `active` attribute.
    /// [`MediaStream.active`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/active)
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }

}
impl MediaStream {
    /// Getter of the `onaddtrack` attribute.
    /// [`MediaStream.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    /// Setter of the `onaddtrack` attribute.
    /// [`MediaStream.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)
    pub fn set_onaddtrack(&mut self, value: &Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl MediaStream {
    /// Getter of the `onremovetrack` attribute.
    /// [`MediaStream.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    /// Setter of the `onremovetrack` attribute.
    /// [`MediaStream.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)
    pub fn set_onremovetrack(&mut self, value: &Any) {
        self.inner.set("onremovetrack", value);
    }
}
