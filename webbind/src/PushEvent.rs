use super::*;

/// The PushEvent class.
/// [`PushEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushEvent {
    inner: ExtendableEvent,
}

impl FromVal for PushEvent {
    fn from_val(v: &Any) -> Self {
        PushEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PushEvent> for Any {
    fn from(s: PushEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushEvent> for Any {
    fn from(s: &PushEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PushEvent);

impl PushEvent {
    /// Getter of the `data` attribute.
    /// [`PushEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/data)
    pub fn data(&self) -> PushMessageData {
        self.inner.get("data").as_::<PushMessageData>()
    }
}
impl PushEvent {
    /// Getter of the `notification` attribute.
    /// [`PushEvent.notification`](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/notification)
    pub fn notification(&self) -> Notification {
        self.inner.get("notification").as_::<Notification>()
    }
}

impl PushEvent {
    /// The `new PushEvent(..)` constructor, creating a new PushEvent instance
    pub fn new(type_: &JsString) -> PushEvent {
        Self {
            inner: Any::global("PushEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}

impl PushEvent {
    /// The `new PushEvent(..)` constructor, creating a new PushEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &PushEventInit,
    ) -> PushEvent {
        Self {
            inner: Any::global("PushEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
