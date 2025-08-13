use super::*;




/// The PresentationConnectionCloseEvent class.
/// [`PresentationConnectionCloseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionCloseEvent {
    inner: Event,
}

impl FromVal for PresentationConnectionCloseEvent {
    fn from_val(v: &Any) -> Self {
        PresentationConnectionCloseEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PresentationConnectionCloseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PresentationConnectionCloseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PresentationConnectionCloseEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PresentationConnectionCloseEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PresentationConnectionCloseEvent> for Any {
    fn from(s: PresentationConnectionCloseEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PresentationConnectionCloseEvent> for Any {
    fn from(s: &PresentationConnectionCloseEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PresentationConnectionCloseEvent);



impl PresentationConnectionCloseEvent {
    /// The `new PresentationConnectionCloseEvent(..)` constructor, creating a new PresentationConnectionCloseEvent instance
    pub fn new(type_: &JsString, event_init_dict: &PresentationConnectionCloseEventInit) -> PresentationConnectionCloseEvent {
        Self {
            inner: Any::global("PresentationConnectionCloseEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl PresentationConnectionCloseEvent {
    /// Getter of the `reason` attribute.
    /// [`PresentationConnectionCloseEvent.reason`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)
    pub fn reason(&self) -> PresentationConnectionCloseReason {
        self.inner.get("reason").as_::<PresentationConnectionCloseReason>()
    }

}
impl PresentationConnectionCloseEvent {
    /// Getter of the `message` attribute.
    /// [`PresentationConnectionCloseEvent.message`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

}
