use super::*;

/// The USBConnectionEvent class.
/// [`USBConnectionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBConnectionEvent {
    inner: Event,
}
impl FromVal for USBConnectionEvent {
    fn from_val(v: &Any) -> Self {
        USBConnectionEvent {
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
impl core::ops::Deref for USBConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBConnectionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBConnectionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBConnectionEvent> for Any {
    fn from(s: USBConnectionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBConnectionEvent> for Any {
    fn from(s: &USBConnectionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBConnectionEvent);

impl USBConnectionEvent {
    /// The `new USBConnectionEvent(..)` constructor, creating a new USBConnectionEvent instance
    pub fn new(type_: &DOMString, event_init_dict: &Any) -> USBConnectionEvent {
        Self {
            inner: Any::global("USBConnectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl USBConnectionEvent {
    /// Getter of the `device` attribute.
    /// [`USBConnectionEvent.device`](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent/device)
    pub fn device(&self) -> USBDevice {
        self.inner.get("device").as_::<USBDevice>()
    }
}
