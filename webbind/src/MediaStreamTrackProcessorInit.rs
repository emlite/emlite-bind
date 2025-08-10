use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackProcessorInit {
    inner: Any,
}
impl FromVal for MediaStreamTrackProcessorInit {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackProcessorInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaStreamTrackProcessorInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamTrackProcessorInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamTrackProcessorInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamTrackProcessorInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamTrackProcessorInit> for Any {
    fn from(s: MediaStreamTrackProcessorInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamTrackProcessorInit> for Any {
    fn from(s: &MediaStreamTrackProcessorInit) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamTrackProcessorInit {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }

    pub fn set_track(&mut self, value: &MediaStreamTrack) {
        self.inner.set("track", value);
    }
}
impl MediaStreamTrackProcessorInit {
    pub fn max_buffer_size(&self) -> u16 {
        self.inner.get("maxBufferSize").as_::<u16>()
    }

    pub fn set_max_buffer_size(&mut self, value: u16) {
        self.inner.set("maxBufferSize", value);
    }
}
