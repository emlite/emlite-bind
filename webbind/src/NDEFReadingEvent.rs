use super::*;

/// The NDEFReadingEvent class.
/// [`NDEFReadingEvent`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReadingEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFReadingEvent {
    inner: Event,
}
impl FromVal for NDEFReadingEvent {
    fn from_val(v: &Any) -> Self {
        NDEFReadingEvent {
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
impl core::ops::Deref for NDEFReadingEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFReadingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFReadingEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFReadingEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFReadingEvent> for Any {
    fn from(s: NDEFReadingEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFReadingEvent> for Any {
    fn from(s: &NDEFReadingEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NDEFReadingEvent);

impl NDEFReadingEvent {
    /// The `new NDEFReadingEvent(..)` constructor, creating a new NDEFReadingEvent instance
    pub fn new(
        type_: &JsString,
        reading_event_init_dict: &NDEFReadingEventInit,
    ) -> NDEFReadingEvent {
        Self {
            inner: Any::global("NDEFReadingEvent")
                .new(&[type_.into(), reading_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NDEFReadingEvent {
    /// Getter of the `serialNumber` attribute.
    /// [`NDEFReadingEvent.serialNumber`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReadingEvent/serialNumber)
    pub fn serial_number(&self) -> JsString {
        self.inner.get("serialNumber").as_::<JsString>()
    }
}
impl NDEFReadingEvent {
    /// Getter of the `message` attribute.
    /// [`NDEFReadingEvent.message`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReadingEvent/message)
    pub fn message(&self) -> NDEFMessage {
        self.inner.get("message").as_::<NDEFMessage>()
    }
}
