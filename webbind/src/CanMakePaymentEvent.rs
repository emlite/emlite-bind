use super::*;

#[derive(Clone, Debug)]
pub struct CanMakePaymentEvent {
    inner: ExtendableEvent,
}
impl FromVal for CanMakePaymentEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CanMakePaymentEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanMakePaymentEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanMakePaymentEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanMakePaymentEvent> for emlite::Val {
    fn from(s: CanMakePaymentEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanMakePaymentEvent {
    pub fn new(type_: jsbind::DOMString) -> CanMakePaymentEvent {
        Self {
            inner: emlite::Val::global("CanMakePaymentEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl CanMakePaymentEvent {
    pub fn respond_with(&self, can_make_payment_response: jsbind::Promise) -> jsbind::Undefined {
        self.inner
            .call("respondWith", &[can_make_payment_response.into()])
            .as_::<jsbind::Undefined>()
    }
}
