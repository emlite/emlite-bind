use super::*;




/// The SensorErrorEvent class.
/// [`SensorErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SensorErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SensorErrorEvent {
    inner: Event,
}

impl FromVal for SensorErrorEvent {
    fn from_val(v: &Any) -> Self {
        SensorErrorEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SensorErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SensorErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SensorErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SensorErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SensorErrorEvent> for Any {
    fn from(s: SensorErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SensorErrorEvent> for Any {
    fn from(s: &SensorErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SensorErrorEvent);



impl SensorErrorEvent {
    /// The `new SensorErrorEvent(..)` constructor, creating a new SensorErrorEvent instance
    pub fn new(type_: &JsString, error_event_init_dict: &SensorErrorEventInit) -> SensorErrorEvent {
        Self {
            inner: Any::global("SensorErrorEvent").new(&[type_.into(), error_event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl SensorErrorEvent {
    /// Getter of the `error` attribute.
    /// [`SensorErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/SensorErrorEvent/error)
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }

}
