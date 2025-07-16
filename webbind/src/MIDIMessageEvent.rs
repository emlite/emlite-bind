use super::*;

/// The MIDIMessageEvent class.
/// [`MIDIMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIMessageEvent {
    inner: Event,
}
impl FromVal for MIDIMessageEvent {
    fn from_val(v: &Any) -> Self {
        MIDIMessageEvent {
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
impl core::ops::Deref for MIDIMessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIMessageEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIMessageEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIMessageEvent> for Any {
    fn from(s: MIDIMessageEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIMessageEvent> for Any {
    fn from(s: &MIDIMessageEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIMessageEvent);

impl MIDIMessageEvent {
    /// The `new MIDIMessageEvent(..)` constructor, creating a new MIDIMessageEvent instance
    pub fn new0(type_: &str) -> MIDIMessageEvent {
        Self {
            inner: Any::global("MIDIMessageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new MIDIMessageEvent(..)` constructor, creating a new MIDIMessageEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> MIDIMessageEvent {
        Self {
            inner: Any::global("MIDIMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MIDIMessageEvent {
    /// Getter of the `data` attribute.
    /// [`MIDIMessageEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/data)
    pub fn data(&self) -> Uint8Array {
        self.inner.get("data").as_::<Uint8Array>()
    }
}
