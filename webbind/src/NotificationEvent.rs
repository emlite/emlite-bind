use super::*;




/// The NotificationEvent class.
/// [`NotificationEvent`](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationEvent {
    inner: ExtendableEvent,
}

impl FromVal for NotificationEvent {
    fn from_val(v: &Any) -> Self {
        NotificationEvent { inner: ExtendableEvent::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotificationEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotificationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotificationEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotificationEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NotificationEvent> for Any {
    fn from(s: NotificationEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotificationEvent> for Any {
    fn from(s: &NotificationEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NotificationEvent);



impl NotificationEvent {
    /// The `new NotificationEvent(..)` constructor, creating a new NotificationEvent instance
    pub fn new(type_: &JsString, event_init_dict: &NotificationEventInit) -> NotificationEvent {
        Self {
            inner: Any::global("NotificationEvent").new(&[type_.into(), event_init_dict.into()]).as_::<ExtendableEvent>(),
        }
    }

}
impl NotificationEvent {
    /// Getter of the `notification` attribute.
    /// [`NotificationEvent.notification`](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/notification)
    pub fn notification(&self) -> Notification {
        self.inner.get("notification").as_::<Notification>()
    }

}
impl NotificationEvent {
    /// Getter of the `action` attribute.
    /// [`NotificationEvent.action`](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/action)
    pub fn action(&self) -> JsString {
        self.inner.get("action").as_::<JsString>()
    }

}
