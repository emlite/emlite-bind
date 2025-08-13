use super::*;




/// The RTCErrorEvent class.
/// [`RTCErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCErrorEvent {
    inner: Event,
}

impl FromVal for RTCErrorEvent {
    fn from_val(v: &Any) -> Self {
        RTCErrorEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCErrorEvent> for Any {
    fn from(s: RTCErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCErrorEvent> for Any {
    fn from(s: &RTCErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCErrorEvent);



impl RTCErrorEvent {
    /// The `new RTCErrorEvent(..)` constructor, creating a new RTCErrorEvent instance
    pub fn new(type_: &JsString, event_init_dict: &RTCErrorEventInit) -> RTCErrorEvent {
        Self {
            inner: Any::global("RTCErrorEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl RTCErrorEvent {
    /// Getter of the `error` attribute.
    /// [`RTCErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/RTCErrorEvent/error)
    pub fn error(&self) -> RTCError {
        self.inner.get("error").as_::<RTCError>()
    }

}
