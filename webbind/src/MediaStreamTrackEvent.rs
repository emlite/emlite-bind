use super::*;

/// The MediaStreamTrackEvent class.
/// [`MediaStreamTrackEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackEvent {
    inner: Event,
}

impl FromVal for MediaStreamTrackEvent {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaStreamTrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamTrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamTrackEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamTrackEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaStreamTrackEvent> for Any {
    fn from(s: MediaStreamTrackEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamTrackEvent> for Any {
    fn from(s: &MediaStreamTrackEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaStreamTrackEvent);

impl MediaStreamTrackEvent {
    /// Getter of the `track` attribute.
    /// [`MediaStreamTrackEvent.track`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}

impl MediaStreamTrackEvent {
    /// The `new MediaStreamTrackEvent(..)` constructor, creating a new MediaStreamTrackEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &MediaStreamTrackEventInit,
    ) -> MediaStreamTrackEvent {
        Self {
            inner: Any::global("MediaStreamTrackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
