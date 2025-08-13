use super::*;




/// The InstallEvent class.
/// [`InstallEvent`](https://developer.mozilla.org/en-US/docs/Web/API/InstallEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InstallEvent {
    inner: ExtendableEvent,
}

impl FromVal for InstallEvent {
    fn from_val(v: &Any) -> Self {
        InstallEvent { inner: ExtendableEvent::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InstallEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InstallEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InstallEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InstallEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InstallEvent> for Any {
    fn from(s: InstallEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InstallEvent> for Any {
    fn from(s: &InstallEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InstallEvent);



impl InstallEvent {
    /// The `new InstallEvent(..)` constructor, creating a new InstallEvent instance
    pub fn new0(type_: &JsString) -> InstallEvent {
        Self {
            inner: Any::global("InstallEvent").new(&[type_.into()]).as_::<ExtendableEvent>(),
        }
    }

    /// The `new InstallEvent(..)` constructor, creating a new InstallEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &ExtendableEventInit) -> InstallEvent {
        Self {
            inner: Any::global("InstallEvent").new(&[type_.into(), event_init_dict.into()]).as_::<ExtendableEvent>(),
        }
    }

}
impl InstallEvent {
    /// The addRoutes method.
    /// [`InstallEvent.addRoutes`](https://developer.mozilla.org/en-US/docs/Web/API/InstallEvent/addRoutes)
    pub fn add_routes(&self, rules: &Any) -> Promise<Undefined> {
        self.inner.call("addRoutes", &[rules.into(), ]).as_::<Promise<Undefined>>()
    }
}
