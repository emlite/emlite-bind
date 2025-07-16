use super::*;

/// The PageRevealEvent class.
/// [`PageRevealEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageRevealEvent {
    inner: Event,
}
impl FromVal for PageRevealEvent {
    fn from_val(v: &Any) -> Self {
        PageRevealEvent {
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
impl core::ops::Deref for PageRevealEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PageRevealEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PageRevealEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PageRevealEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PageRevealEvent> for Any {
    fn from(s: PageRevealEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PageRevealEvent> for Any {
    fn from(s: &PageRevealEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PageRevealEvent);

impl PageRevealEvent {
    /// The `new PageRevealEvent(..)` constructor, creating a new PageRevealEvent instance
    pub fn new0(type_: &str) -> PageRevealEvent {
        Self {
            inner: Any::global("PageRevealEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new PageRevealEvent(..)` constructor, creating a new PageRevealEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> PageRevealEvent {
        Self {
            inner: Any::global("PageRevealEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PageRevealEvent {
    /// Getter of the `viewTransition` attribute.
    /// [`PageRevealEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent/viewTransition)
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }
}
