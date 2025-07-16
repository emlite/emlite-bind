use super::*;

/// The ToggleEvent class.
/// [`ToggleEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ToggleEvent {
    inner: Event,
}
impl FromVal for ToggleEvent {
    fn from_val(v: &Any) -> Self {
        ToggleEvent {
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
impl core::ops::Deref for ToggleEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ToggleEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ToggleEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ToggleEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ToggleEvent> for Any {
    fn from(s: ToggleEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ToggleEvent> for Any {
    fn from(s: &ToggleEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ToggleEvent);

impl ToggleEvent {
    /// The `new ToggleEvent(..)` constructor, creating a new ToggleEvent instance
    pub fn new0(type_: &str) -> ToggleEvent {
        Self {
            inner: Any::global("ToggleEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new ToggleEvent(..)` constructor, creating a new ToggleEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> ToggleEvent {
        Self {
            inner: Any::global("ToggleEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ToggleEvent {
    /// Getter of the `oldState` attribute.
    /// [`ToggleEvent.oldState`](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/oldState)
    pub fn old_state(&self) -> String {
        self.inner.get("oldState").as_::<String>()
    }
}
impl ToggleEvent {
    /// Getter of the `newState` attribute.
    /// [`ToggleEvent.newState`](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/newState)
    pub fn new_state(&self) -> String {
        self.inner.get("newState").as_::<String>()
    }
}
impl ToggleEvent {
    /// Getter of the `source` attribute.
    /// [`ToggleEvent.source`](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/source)
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
