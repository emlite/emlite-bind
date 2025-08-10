use super::*;

/// The PaymentRequestUpdateEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestUpdateEventInit {
    inner: Any,
}

impl FromVal for PaymentRequestUpdateEventInit {
    fn from_val(v: &Any) -> Self {
        PaymentRequestUpdateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentRequestUpdateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentRequestUpdateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentRequestUpdateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentRequestUpdateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentRequestUpdateEventInit> for Any {
    fn from(s: PaymentRequestUpdateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentRequestUpdateEventInit> for Any {
    fn from(s: &PaymentRequestUpdateEventInit) -> Any {
        s.inner.clone()
    }
}
