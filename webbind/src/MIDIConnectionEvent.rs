use super::*;

/// The MIDIConnectionEvent class.
/// [`MIDIConnectionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIConnectionEvent {
    inner: Event,
}
impl FromVal for MIDIConnectionEvent {
    fn from_val(v: &Any) -> Self {
        MIDIConnectionEvent {
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
impl core::ops::Deref for MIDIConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIConnectionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIConnectionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIConnectionEvent> for Any {
    fn from(s: MIDIConnectionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIConnectionEvent> for Any {
    fn from(s: &MIDIConnectionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIConnectionEvent);

impl MIDIConnectionEvent {
    /// The `new MIDIConnectionEvent(..)` constructor, creating a new MIDIConnectionEvent instance
    pub fn new0(type_: &JsString) -> MIDIConnectionEvent {
        Self {
            inner: Any::global("MIDIConnectionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new MIDIConnectionEvent(..)` constructor, creating a new MIDIConnectionEvent instance
    pub fn new1(
        type_: &JsString,
        event_init_dict: &MIDIConnectionEventInit,
    ) -> MIDIConnectionEvent {
        Self {
            inner: Any::global("MIDIConnectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MIDIConnectionEvent {
    /// Getter of the `port` attribute.
    /// [`MIDIConnectionEvent.port`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)
    pub fn port(&self) -> MIDIPort {
        self.inner.get("port").as_::<MIDIPort>()
    }
}
