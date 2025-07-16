use super::*;

/// The PaymentRequestUpdateEvent class.
/// [`PaymentRequestUpdateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestUpdateEvent {
    inner: Event,
}
impl FromVal for PaymentRequestUpdateEvent {
    fn from_val(v: &Any) -> Self {
        PaymentRequestUpdateEvent {
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
impl core::ops::Deref for PaymentRequestUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentRequestUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentRequestUpdateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentRequestUpdateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentRequestUpdateEvent> for Any {
    fn from(s: PaymentRequestUpdateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentRequestUpdateEvent> for Any {
    fn from(s: &PaymentRequestUpdateEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentRequestUpdateEvent);

impl PaymentRequestUpdateEvent {
    /// The `new PaymentRequestUpdateEvent(..)` constructor, creating a new PaymentRequestUpdateEvent instance
    pub fn new0(type_: &str) -> PaymentRequestUpdateEvent {
        Self {
            inner: Any::global("PaymentRequestUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new PaymentRequestUpdateEvent(..)` constructor, creating a new PaymentRequestUpdateEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> PaymentRequestUpdateEvent {
        Self {
            inner: Any::global("PaymentRequestUpdateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PaymentRequestUpdateEvent {
    /// The updateWith method.
    /// [`PaymentRequestUpdateEvent.updateWith`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/updateWith)
    pub fn update_with(&self, details_promise: &Promise) -> Undefined {
        self.inner
            .call("updateWith", &[details_promise.into()])
            .as_::<Undefined>()
    }
}
