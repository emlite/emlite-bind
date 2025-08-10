use super::*;

/// The PresentationConnectionAvailableEvent class.
/// [`PresentationConnectionAvailableEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionAvailableEvent {
    inner: Event,
}
impl FromVal for PresentationConnectionAvailableEvent {
    fn from_val(v: &Any) -> Self {
        PresentationConnectionAvailableEvent {
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
impl core::ops::Deref for PresentationConnectionAvailableEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnectionAvailableEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PresentationConnectionAvailableEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PresentationConnectionAvailableEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PresentationConnectionAvailableEvent> for Any {
    fn from(s: PresentationConnectionAvailableEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PresentationConnectionAvailableEvent> for Any {
    fn from(s: &PresentationConnectionAvailableEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PresentationConnectionAvailableEvent);

impl PresentationConnectionAvailableEvent {
    /// The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new PresentationConnectionAvailableEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &PresentationConnectionAvailableEventInit,
    ) -> PresentationConnectionAvailableEvent {
        Self {
            inner: Any::global("PresentationConnectionAvailableEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PresentationConnectionAvailableEvent {
    /// Getter of the `connection` attribute.
    /// [`PresentationConnectionAvailableEvent.connection`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)
    pub fn connection(&self) -> PresentationConnection {
        self.inner.get("connection").as_::<PresentationConnection>()
    }
}
