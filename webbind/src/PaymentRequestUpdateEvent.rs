use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestUpdateEvent {
    inner: Event,
}
impl FromVal for PaymentRequestUpdateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentRequestUpdateEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for PaymentRequestUpdateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentRequestUpdateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PaymentRequestUpdateEvent> for emlite::Val {
    fn from(s: PaymentRequestUpdateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PaymentRequestUpdateEvent);



impl PaymentRequestUpdateEvent {
    pub fn new0(type_: DOMString) -> PaymentRequestUpdateEvent {
        Self {
            inner: emlite::Val::global("PaymentRequestUpdateEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PaymentRequestUpdateEvent {
        Self {
            inner: emlite::Val::global("PaymentRequestUpdateEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl PaymentRequestUpdateEvent {
    pub fn update_with(&self, details_promise: Promise) -> Undefined {
        self.inner.call("updateWith", &[details_promise.into(), ]).as_::<Undefined>()
    }

}
