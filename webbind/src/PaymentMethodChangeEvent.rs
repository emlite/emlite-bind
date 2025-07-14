use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentMethodChangeEvent {
    inner: PaymentRequestUpdateEvent,
}
impl FromVal for PaymentMethodChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentMethodChangeEvent {
            inner: PaymentRequestUpdateEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for PaymentMethodChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentMethodChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PaymentMethodChangeEvent> for emlite::Val {
    fn from(s: PaymentMethodChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PaymentMethodChangeEvent);

impl PaymentMethodChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> PaymentMethodChangeEvent {
        Self {
            inner: emlite::Val::global("PaymentMethodChangeEvent")
                .new(&[type_.into()])
                .as_::<PaymentRequestUpdateEvent>(),
        }
    }

    pub fn new1(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> PaymentMethodChangeEvent {
        Self {
            inner: emlite::Val::global("PaymentMethodChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<PaymentRequestUpdateEvent>(),
        }
    }
}
impl PaymentMethodChangeEvent {
    pub fn method_name(&self) -> jsbind::DOMString {
        self.inner.get("methodName").as_::<jsbind::DOMString>()
    }
}
impl PaymentMethodChangeEvent {
    pub fn method_details(&self) -> jsbind::Object {
        self.inner.get("methodDetails").as_::<jsbind::Object>()
    }
}
