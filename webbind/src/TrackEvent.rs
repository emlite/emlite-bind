use super::*;

/// The TrackEvent class.
/// [`TrackEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrackEvent {
    inner: Event,
}

impl FromVal for TrackEvent {
    fn from_val(v: &Any) -> Self {
        TrackEvent {
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

impl core::ops::Deref for TrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TrackEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TrackEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TrackEvent> for Any {
    fn from(s: TrackEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TrackEvent> for Any {
    fn from(s: &TrackEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TrackEvent);

impl TrackEvent {
    /// Getter of the `track` attribute.
    /// [`TrackEvent.track`](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/track)
    pub fn track(&self) -> Any {
        self.inner.get("track").as_::<Any>()
    }
}

impl TrackEvent {
    /// The `new TrackEvent(..)` constructor, creating a new TrackEvent instance
    pub fn new0(type_: &JsString) -> TrackEvent {
        Self {
            inner: Any::global("TrackEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new TrackEvent(..)` constructor, creating a new TrackEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &TrackEventInit) -> TrackEvent {
        Self {
            inner: Any::global("TrackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
