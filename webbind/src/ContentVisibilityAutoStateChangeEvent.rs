use super::*;

/// The ContentVisibilityAutoStateChangeEvent class.
/// [`ContentVisibilityAutoStateChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ContentVisibilityAutoStateChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentVisibilityAutoStateChangeEvent {
    inner: Event,
}

impl FromVal for ContentVisibilityAutoStateChangeEvent {
    fn from_val(v: &Any) -> Self {
        ContentVisibilityAutoStateChangeEvent {
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

impl core::ops::Deref for ContentVisibilityAutoStateChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContentVisibilityAutoStateChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContentVisibilityAutoStateChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContentVisibilityAutoStateChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ContentVisibilityAutoStateChangeEvent> for Any {
    fn from(s: ContentVisibilityAutoStateChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContentVisibilityAutoStateChangeEvent> for Any {
    fn from(s: &ContentVisibilityAutoStateChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ContentVisibilityAutoStateChangeEvent);

impl ContentVisibilityAutoStateChangeEvent {
    /// Getter of the `skipped` attribute.
    /// [`ContentVisibilityAutoStateChangeEvent.skipped`](https://developer.mozilla.org/en-US/docs/Web/API/ContentVisibilityAutoStateChangeEvent/skipped)
    pub fn skipped(&self) -> bool {
        self.inner.get("skipped").as_::<bool>()
    }
}

impl ContentVisibilityAutoStateChangeEvent {
    /// The `new ContentVisibilityAutoStateChangeEvent(..)` constructor, creating a new ContentVisibilityAutoStateChangeEvent instance
    pub fn new(type_: &JsString) -> ContentVisibilityAutoStateChangeEvent {
        Self {
            inner: Any::global("ContentVisibilityAutoStateChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl ContentVisibilityAutoStateChangeEvent {
    /// The `new ContentVisibilityAutoStateChangeEvent(..)` constructor, creating a new ContentVisibilityAutoStateChangeEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &ContentVisibilityAutoStateChangeEventInit,
    ) -> ContentVisibilityAutoStateChangeEvent {
        Self {
            inner: Any::global("ContentVisibilityAutoStateChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
