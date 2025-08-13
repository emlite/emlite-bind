use super::*;




/// The MediaSession class.
/// [`MediaSession`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaSession {
    inner: Any,
}

impl FromVal for MediaSession {
    fn from_val(v: &Any) -> Self {
        MediaSession { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaSession {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaSession {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaSession {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaSession> for Any {
    fn from(s: MediaSession) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaSession> for Any {
    fn from(s: &MediaSession) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaSession);


impl MediaSession {
    /// Getter of the `metadata` attribute.
    /// [`MediaSession.metadata`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)
    pub fn metadata(&self) -> MediaMetadata {
        self.inner.get("metadata").as_::<MediaMetadata>()
    }

    /// Setter of the `metadata` attribute.
    /// [`MediaSession.metadata`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)
    pub fn set_metadata(&mut self, value: &MediaMetadata) {
        self.inner.set("metadata", value);
    }
}
impl MediaSession {
    /// Getter of the `playbackState` attribute.
    /// [`MediaSession.playbackState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)
    pub fn playback_state(&self) -> MediaSessionPlaybackState {
        self.inner.get("playbackState").as_::<MediaSessionPlaybackState>()
    }

    /// Setter of the `playbackState` attribute.
    /// [`MediaSession.playbackState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)
    pub fn set_playback_state(&mut self, value: &MediaSessionPlaybackState) {
        self.inner.set("playbackState", value);
    }
}
impl MediaSession {
    /// The setActionHandler method.
    /// [`MediaSession.setActionHandler`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setActionHandler)
    pub fn set_action_handler(&self, action: &MediaSessionAction, handler: &Function) -> Undefined {
        self.inner.call("setActionHandler", &[action.into(), handler.into(), ]).as_::<Undefined>()
    }
}
impl MediaSession {
    /// The setPositionState method.
    /// [`MediaSession.setPositionState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)
    pub fn set_position_state0(&self, ) -> Undefined {
        self.inner.call("setPositionState", &[]).as_::<Undefined>()
    }
    /// The setPositionState method.
    /// [`MediaSession.setPositionState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)
    pub fn set_position_state1(&self, state: &MediaPositionState) -> Undefined {
        self.inner.call("setPositionState", &[state.into(), ]).as_::<Undefined>()
    }
}
impl MediaSession {
    /// The setMicrophoneActive method.
    /// [`MediaSession.setMicrophoneActive`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setMicrophoneActive)
    pub fn set_microphone_active(&self, active: bool) -> Promise<Undefined> {
        self.inner.call("setMicrophoneActive", &[active.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl MediaSession {
    /// The setCameraActive method.
    /// [`MediaSession.setCameraActive`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setCameraActive)
    pub fn set_camera_active(&self, active: bool) -> Promise<Undefined> {
        self.inner.call("setCameraActive", &[active.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl MediaSession {
    /// The setScreenshareActive method.
    /// [`MediaSession.setScreenshareActive`](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setScreenshareActive)
    pub fn set_screenshare_active(&self, active: bool) -> Promise<Undefined> {
        self.inner.call("setScreenshareActive", &[active.into(), ]).as_::<Promise<Undefined>>()
    }
}
