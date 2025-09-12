use super::*;

/// The VideoTrackGenerator class.
/// [`VideoTrackGenerator`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackGenerator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoTrackGenerator {
    inner: Any,
}

impl FromVal for VideoTrackGenerator {
    fn from_val(v: &Any) -> Self {
        VideoTrackGenerator {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoTrackGenerator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoTrackGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoTrackGenerator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoTrackGenerator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoTrackGenerator> for Any {
    fn from(s: VideoTrackGenerator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoTrackGenerator> for Any {
    fn from(s: &VideoTrackGenerator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VideoTrackGenerator);

impl VideoTrackGenerator {
    /// Getter of the `writable` attribute.
    /// [`VideoTrackGenerator.writable`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackGenerator/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl VideoTrackGenerator {
    /// Getter of the `muted` attribute.
    /// [`VideoTrackGenerator.muted`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackGenerator/muted)
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }

    /// Setter of the `muted` attribute.
    /// [`VideoTrackGenerator.muted`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackGenerator/muted)
    pub fn set_muted(&mut self, value: bool) {
        self.inner.set("muted", value);
    }
}
impl VideoTrackGenerator {
    /// Getter of the `track` attribute.
    /// [`VideoTrackGenerator.track`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackGenerator/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}

impl VideoTrackGenerator {
    /// The `new VideoTrackGenerator(..)` constructor, creating a new VideoTrackGenerator instance
    pub fn new() -> VideoTrackGenerator {
        Self {
            inner: Any::global("VideoTrackGenerator").new(&[]).as_::<Any>(),
        }
    }
}
