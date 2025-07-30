use super::*;

/// The CanMakePaymentEvent class.
/// [`CanMakePaymentEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CanMakePaymentEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanMakePaymentEvent {
    inner: ExtendableEvent,
}
impl FromVal for CanMakePaymentEvent {
    fn from_val(v: &Any) -> Self {
        CanMakePaymentEvent {
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
impl core::ops::Deref for CanMakePaymentEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanMakePaymentEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CanMakePaymentEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CanMakePaymentEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CanMakePaymentEvent> for Any {
    fn from(s: CanMakePaymentEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CanMakePaymentEvent> for Any {
    fn from(s: &CanMakePaymentEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanMakePaymentEvent);

impl CanMakePaymentEvent {
    /// The `new CanMakePaymentEvent(..)` constructor, creating a new CanMakePaymentEvent instance
    pub fn new(type_: &JsString) -> CanMakePaymentEvent {
        Self {
            inner: Any::global("CanMakePaymentEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl CanMakePaymentEvent {
    /// The respondWith method.
    /// [`CanMakePaymentEvent.respondWith`](https://developer.mozilla.org/en-US/docs/Web/API/CanMakePaymentEvent/respondWith)
    pub fn respond_with(&self, can_make_payment_response: Promise<bool>) -> Undefined {
        self.inner
            .call("respondWith", &[can_make_payment_response.into()])
            .as_::<Undefined>()
    }
}
