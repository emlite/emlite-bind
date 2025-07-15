use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for CanMakePaymentEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanMakePaymentEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CanMakePaymentEvent> for emlite::Val {
    fn from(s: CanMakePaymentEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CanMakePaymentEvent> for emlite::Val {
    fn from(s: &CanMakePaymentEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanMakePaymentEvent);

impl CanMakePaymentEvent {
    pub fn new(type_: &str) -> CanMakePaymentEvent {
        Self {
            inner: emlite::Val::global("CanMakePaymentEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl CanMakePaymentEvent {
    pub fn respond_with(&self, can_make_payment_response: &Promise) -> Undefined {
        self.inner
            .call("respondWith", &[can_make_payment_response.into()])
            .as_::<Undefined>()
    }
}
