use super::*;

/// The MediaQueryListEvent class.
/// [`MediaQueryListEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaQueryListEvent {
    inner: Event,
}
impl FromVal for MediaQueryListEvent {
    fn from_val(v: &Any) -> Self {
        MediaQueryListEvent {
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
impl core::ops::Deref for MediaQueryListEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaQueryListEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaQueryListEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaQueryListEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaQueryListEvent> for Any {
    fn from(s: MediaQueryListEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaQueryListEvent> for Any {
    fn from(s: &MediaQueryListEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaQueryListEvent);

impl MediaQueryListEvent {
    /// The `new MediaQueryListEvent(..)` constructor, creating a new MediaQueryListEvent instance
    pub fn new0(type_: &JsString) -> MediaQueryListEvent {
        Self {
            inner: Any::global("MediaQueryListEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new MediaQueryListEvent(..)` constructor, creating a new MediaQueryListEvent instance
    pub fn new1(
        type_: &JsString,
        event_init_dict: &MediaQueryListEventInit,
    ) -> MediaQueryListEvent {
        Self {
            inner: Any::global("MediaQueryListEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MediaQueryListEvent {
    /// Getter of the `media` attribute.
    /// [`MediaQueryListEvent.media`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }
}
impl MediaQueryListEvent {
    /// Getter of the `matches` attribute.
    /// [`MediaQueryListEvent.matches`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
