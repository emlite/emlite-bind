use super::*;

/// The RTCDTMFToneChangeEvent class.
/// [`RTCDTMFToneChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDTMFToneChangeEvent {
    inner: Event,
}
impl FromVal for RTCDTMFToneChangeEvent {
    fn from_val(v: &Any) -> Self {
        RTCDTMFToneChangeEvent {
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
impl core::ops::Deref for RTCDTMFToneChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDTMFToneChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDTMFToneChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDTMFToneChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDTMFToneChangeEvent> for Any {
    fn from(s: RTCDTMFToneChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDTMFToneChangeEvent> for Any {
    fn from(s: &RTCDTMFToneChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCDTMFToneChangeEvent);

impl RTCDTMFToneChangeEvent {
    /// The `new RTCDTMFToneChangeEvent(..)` constructor, creating a new RTCDTMFToneChangeEvent instance
    pub fn new0(type_: &DOMString) -> RTCDTMFToneChangeEvent {
        Self {
            inner: Any::global("RTCDTMFToneChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new RTCDTMFToneChangeEvent(..)` constructor, creating a new RTCDTMFToneChangeEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> RTCDTMFToneChangeEvent {
        Self {
            inner: Any::global("RTCDTMFToneChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCDTMFToneChangeEvent {
    /// Getter of the `tone` attribute.
    /// [`RTCDTMFToneChangeEvent.tone`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)
    pub fn tone(&self) -> DOMString {
        self.inner.get("tone").as_::<DOMString>()
    }
}
