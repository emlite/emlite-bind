use super::*;




/// The MediaStreamTrackEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackEventInit {
    inner: Any,
}

impl FromVal for MediaStreamTrackEventInit {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaStreamTrackEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamTrackEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamTrackEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamTrackEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaStreamTrackEventInit> for Any {
    fn from(s: MediaStreamTrackEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamTrackEventInit> for Any {
    fn from(s: &MediaStreamTrackEventInit) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamTrackEventInit {
    /// Getter of the `track` attribute.
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }

    /// Setter of the `track` attribute.
    pub fn set_track(&mut self, value: &MediaStreamTrack) {
        self.inner.set("track", value);
    }
}
