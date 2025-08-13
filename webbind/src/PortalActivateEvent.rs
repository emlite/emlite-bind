use super::*;




/// The PortalActivateEvent class.
/// [`PortalActivateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PortalActivateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PortalActivateEvent {
    inner: Event,
}

impl FromVal for PortalActivateEvent {
    fn from_val(v: &Any) -> Self {
        PortalActivateEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PortalActivateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PortalActivateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PortalActivateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PortalActivateEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PortalActivateEvent> for Any {
    fn from(s: PortalActivateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PortalActivateEvent> for Any {
    fn from(s: &PortalActivateEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PortalActivateEvent);



impl PortalActivateEvent {
    /// The `new PortalActivateEvent(..)` constructor, creating a new PortalActivateEvent instance
    pub fn new0(type_: &JsString) -> PortalActivateEvent {
        Self {
            inner: Any::global("PortalActivateEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new PortalActivateEvent(..)` constructor, creating a new PortalActivateEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &PortalActivateEventInit) -> PortalActivateEvent {
        Self {
            inner: Any::global("PortalActivateEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl PortalActivateEvent {
    /// Getter of the `data` attribute.
    /// [`PortalActivateEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/PortalActivateEvent/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

}
impl PortalActivateEvent {
    /// The adoptPredecessor method.
    /// [`PortalActivateEvent.adoptPredecessor`](https://developer.mozilla.org/en-US/docs/Web/API/PortalActivateEvent/adoptPredecessor)
    pub fn adopt_predecessor(&self, ) -> HTMLPortalElement {
        self.inner.call("adoptPredecessor", &[]).as_::<HTMLPortalElement>()
    }
}
