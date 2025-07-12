use super::*;

#[derive(Clone, Debug)]
pub struct PaymentRequestUpdateEvent {
    inner: Event,
}
impl FromVal for PaymentRequestUpdateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentRequestUpdateEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentRequestUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentRequestUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentRequestUpdateEvent> for emlite::Val {
    fn from(s: PaymentRequestUpdateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentRequestUpdateEvent {
    pub fn new0(type_: jsbind::DOMString) -> PaymentRequestUpdateEvent {
        Self {
            inner: emlite::Val::global("PaymentRequestUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> PaymentRequestUpdateEvent {
        Self {
            inner: emlite::Val::global("PaymentRequestUpdateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PaymentRequestUpdateEvent {
    pub fn update_with(&self, details_promise: jsbind::Promise) -> jsbind::Undefined {
        self.inner
            .call("updateWith", &[details_promise.into()])
            .as_::<jsbind::Undefined>()
    }
}
