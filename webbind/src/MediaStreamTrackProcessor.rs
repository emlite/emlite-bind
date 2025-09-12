use super::*;

/// The MediaStreamTrackProcessor class.
/// [`MediaStreamTrackProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackProcessor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackProcessor {
    inner: Any,
}

impl FromVal for MediaStreamTrackProcessor {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackProcessor {
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

impl core::ops::Deref for MediaStreamTrackProcessor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamTrackProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamTrackProcessor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamTrackProcessor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaStreamTrackProcessor> for Any {
    fn from(s: MediaStreamTrackProcessor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamTrackProcessor> for Any {
    fn from(s: &MediaStreamTrackProcessor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaStreamTrackProcessor);

impl MediaStreamTrackProcessor {
    /// Getter of the `readable` attribute.
    /// [`MediaStreamTrackProcessor.readable`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackProcessor/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}

impl MediaStreamTrackProcessor {
    /// The `new MediaStreamTrackProcessor(..)` constructor, creating a new MediaStreamTrackProcessor instance
    pub fn new(init: &MediaStreamTrackProcessorInit) -> MediaStreamTrackProcessor {
        Self {
            inner: Any::global("MediaStreamTrackProcessor")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
