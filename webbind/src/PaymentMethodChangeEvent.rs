use super::*;

/// The PaymentMethodChangeEvent class.
/// [`PaymentMethodChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentMethodChangeEvent {
    inner: PaymentRequestUpdateEvent,
}
impl FromVal for PaymentMethodChangeEvent {
    fn from_val(v: &Any) -> Self {
        PaymentMethodChangeEvent {
            inner: PaymentRequestUpdateEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentMethodChangeEvent {
    type Target = PaymentRequestUpdateEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentMethodChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentMethodChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentMethodChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentMethodChangeEvent> for Any {
    fn from(s: PaymentMethodChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentMethodChangeEvent> for Any {
    fn from(s: &PaymentMethodChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentMethodChangeEvent);

impl PaymentMethodChangeEvent {
    /// The `new PaymentMethodChangeEvent(..)` constructor, creating a new PaymentMethodChangeEvent instance
    pub fn new0(type_: &DOMString) -> PaymentMethodChangeEvent {
        Self {
            inner: Any::global("PaymentMethodChangeEvent")
                .new(&[type_.into()])
                .as_::<PaymentRequestUpdateEvent>(),
        }
    }

    /// The `new PaymentMethodChangeEvent(..)` constructor, creating a new PaymentMethodChangeEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> PaymentMethodChangeEvent {
        Self {
            inner: Any::global("PaymentMethodChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<PaymentRequestUpdateEvent>(),
        }
    }
}
impl PaymentMethodChangeEvent {
    /// Getter of the `methodName` attribute.
    /// [`PaymentMethodChangeEvent.methodName`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)
    pub fn method_name(&self) -> DOMString {
        self.inner.get("methodName").as_::<DOMString>()
    }
}
impl PaymentMethodChangeEvent {
    /// Getter of the `methodDetails` attribute.
    /// [`PaymentMethodChangeEvent.methodDetails`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)
    pub fn method_details(&self) -> Object {
        self.inner.get("methodDetails").as_::<Object>()
    }
}
