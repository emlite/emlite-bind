use super::*;




/// The MediaPositionState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaPositionState {
    inner: Any,
}

impl FromVal for MediaPositionState {
    fn from_val(v: &Any) -> Self {
        MediaPositionState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaPositionState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaPositionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaPositionState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaPositionState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaPositionState> for Any {
    fn from(s: MediaPositionState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaPositionState> for Any {
    fn from(s: &MediaPositionState) -> Any {
        s.inner.clone()
    }
}

impl MediaPositionState {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: f64) {
        self.inner.set("duration", value);
    }
}
impl MediaPositionState {
    /// Getter of the `playbackRate` attribute.
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    /// Setter of the `playbackRate` attribute.
    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }
}
impl MediaPositionState {
    /// Getter of the `position` attribute.
    pub fn position(&self) -> f64 {
        self.inner.get("position").as_::<f64>()
    }

    /// Setter of the `position` attribute.
    pub fn set_position(&mut self, value: f64) {
        self.inner.set("position", value);
    }
}
