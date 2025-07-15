use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaPositionState {
    inner: emlite::Val,
}
impl FromVal for MediaPositionState {
    fn from_val(v: &emlite::Val) -> Self {
        MediaPositionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaPositionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaPositionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaPositionState {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaPositionState {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaPositionState> for emlite::Val {
    fn from(s: MediaPositionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaPositionState> for emlite::Val {
    fn from(s: &MediaPositionState) -> emlite::Val {
        s.inner.clone()
    }
}

impl MediaPositionState {
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }

    pub fn set_duration(&mut self, value: f64) {
        self.inner.set("duration", value);
    }
}
impl MediaPositionState {
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }
}
impl MediaPositionState {
    pub fn position(&self) -> f64 {
        self.inner.get("position").as_::<f64>()
    }

    pub fn set_position(&mut self, value: f64) {
        self.inner.set("position", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaSession {
    inner: emlite::Val,
}
impl FromVal for MediaSession {
    fn from_val(v: &emlite::Val) -> Self {
        MediaSession {
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
impl core::ops::Deref for MediaSession {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaSession {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaSession {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaSession> for emlite::Val {
    fn from(s: MediaSession) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaSession> for emlite::Val {
    fn from(s: &MediaSession) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaSession);

impl MediaSession {
    pub fn metadata(&self) -> MediaMetadata {
        self.inner.get("metadata").as_::<MediaMetadata>()
    }

    pub fn set_metadata(&mut self, value: &MediaMetadata) {
        self.inner.set("metadata", value);
    }
}
impl MediaSession {
    pub fn playback_state(&self) -> MediaSessionPlaybackState {
        self.inner
            .get("playbackState")
            .as_::<MediaSessionPlaybackState>()
    }

    pub fn set_playback_state(&mut self, value: &MediaSessionPlaybackState) {
        self.inner.set("playbackState", value);
    }
}
impl MediaSession {
    pub fn set_action_handler(&self, action: &MediaSessionAction, handler: &Function) -> Undefined {
        self.inner
            .call("setActionHandler", &[action.into(), handler.into()])
            .as_::<Undefined>()
    }
}
impl MediaSession {
    pub fn set_position_state0(&self) -> Undefined {
        self.inner.call("setPositionState", &[]).as_::<Undefined>()
    }

    pub fn set_position_state1(&self, state: &MediaPositionState) -> Undefined {
        self.inner
            .call("setPositionState", &[state.into()])
            .as_::<Undefined>()
    }
}
impl MediaSession {
    pub fn set_microphone_active(&self, active: bool) -> Promise {
        self.inner
            .call("setMicrophoneActive", &[active.into()])
            .as_::<Promise>()
    }
}
impl MediaSession {
    pub fn set_camera_active(&self, active: bool) -> Promise {
        self.inner
            .call("setCameraActive", &[active.into()])
            .as_::<Promise>()
    }
}
impl MediaSession {
    pub fn set_screenshare_active(&self, active: bool) -> Promise {
        self.inner
            .call("setScreenshareActive", &[active.into()])
            .as_::<Promise>()
    }
}
