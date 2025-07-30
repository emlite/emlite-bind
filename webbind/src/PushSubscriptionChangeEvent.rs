use super::*;

/// The PushSubscriptionChangeEvent class.
/// [`PushSubscriptionChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionChangeEvent {
    inner: ExtendableEvent,
}
impl FromVal for PushSubscriptionChangeEvent {
    fn from_val(v: &Any) -> Self {
        PushSubscriptionChangeEvent {
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
impl core::ops::Deref for PushSubscriptionChangeEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscriptionChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PushSubscriptionChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PushSubscriptionChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PushSubscriptionChangeEvent> for Any {
    fn from(s: PushSubscriptionChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PushSubscriptionChangeEvent> for Any {
    fn from(s: &PushSubscriptionChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushSubscriptionChangeEvent);

impl PushSubscriptionChangeEvent {
    /// The `new PushSubscriptionChangeEvent(..)` constructor, creating a new PushSubscriptionChangeEvent instance
    pub fn new0(type_: &JsString) -> PushSubscriptionChangeEvent {
        Self {
            inner: Any::global("PushSubscriptionChangeEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    /// The `new PushSubscriptionChangeEvent(..)` constructor, creating a new PushSubscriptionChangeEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> PushSubscriptionChangeEvent {
        Self {
            inner: Any::global("PushSubscriptionChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PushSubscriptionChangeEvent {
    /// Getter of the `newSubscription` attribute.
    /// [`PushSubscriptionChangeEvent.newSubscription`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionChangeEvent/newSubscription)
    pub fn new_subscription(&self) -> PushSubscription {
        self.inner.get("newSubscription").as_::<PushSubscription>()
    }
}
impl PushSubscriptionChangeEvent {
    /// Getter of the `oldSubscription` attribute.
    /// [`PushSubscriptionChangeEvent.oldSubscription`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionChangeEvent/oldSubscription)
    pub fn old_subscription(&self) -> PushSubscription {
        self.inner.get("oldSubscription").as_::<PushSubscription>()
    }
}
