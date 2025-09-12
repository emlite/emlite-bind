use super::*;

/// The PageTransitionEvent class.
/// [`PageTransitionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageTransitionEvent {
    inner: Event,
}

impl FromVal for PageTransitionEvent {
    fn from_val(v: &Any) -> Self {
        PageTransitionEvent {
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

impl core::ops::Deref for PageTransitionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PageTransitionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PageTransitionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PageTransitionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PageTransitionEvent> for Any {
    fn from(s: PageTransitionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PageTransitionEvent> for Any {
    fn from(s: &PageTransitionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PageTransitionEvent);

impl PageTransitionEvent {
    /// Getter of the `persisted` attribute.
    /// [`PageTransitionEvent.persisted`](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)
    pub fn persisted(&self) -> bool {
        self.inner.get("persisted").as_::<bool>()
    }
}

impl PageTransitionEvent {
    /// The `new PageTransitionEvent(..)` constructor, creating a new PageTransitionEvent instance
    pub fn new0(type_: &JsString) -> PageTransitionEvent {
        Self {
            inner: Any::global("PageTransitionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new PageTransitionEvent(..)` constructor, creating a new PageTransitionEvent instance
    pub fn new1(
        type_: &JsString,
        event_init_dict: &PageTransitionEventInit,
    ) -> PageTransitionEvent {
        Self {
            inner: Any::global("PageTransitionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
