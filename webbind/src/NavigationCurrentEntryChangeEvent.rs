use super::*;

/// The NavigationCurrentEntryChangeEvent class.
/// [`NavigationCurrentEntryChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationCurrentEntryChangeEvent {
    inner: Event,
}
impl FromVal for NavigationCurrentEntryChangeEvent {
    fn from_val(v: &Any) -> Self {
        NavigationCurrentEntryChangeEvent {
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
impl core::ops::Deref for NavigationCurrentEntryChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationCurrentEntryChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationCurrentEntryChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationCurrentEntryChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationCurrentEntryChangeEvent> for Any {
    fn from(s: NavigationCurrentEntryChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationCurrentEntryChangeEvent> for Any {
    fn from(s: &NavigationCurrentEntryChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationCurrentEntryChangeEvent);

impl NavigationCurrentEntryChangeEvent {
    /// The `new NavigationCurrentEntryChangeEvent(..)` constructor, creating a new NavigationCurrentEntryChangeEvent instance
    pub fn new(type_: &DOMString, event_init_dict: &Any) -> NavigationCurrentEntryChangeEvent {
        Self {
            inner: Any::global("NavigationCurrentEntryChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NavigationCurrentEntryChangeEvent {
    /// Getter of the `navigationType` attribute.
    /// [`NavigationCurrentEntryChangeEvent.navigationType`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/navigationType)
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
impl NavigationCurrentEntryChangeEvent {
    /// Getter of the `from` attribute.
    /// [`NavigationCurrentEntryChangeEvent.from`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/from)
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
