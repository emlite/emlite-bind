use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CollectedClientPaymentData {
    inner: Any,
}
impl FromVal for CollectedClientPaymentData {
    fn from_val(v: &Any) -> Self {
        CollectedClientPaymentData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CollectedClientPaymentData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CollectedClientPaymentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CollectedClientPaymentData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CollectedClientPaymentData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CollectedClientPaymentData> for Any {
    fn from(s: CollectedClientPaymentData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CollectedClientPaymentData> for Any {
    fn from(s: &CollectedClientPaymentData) -> Any {
        s.inner.clone()
    }
}

impl CollectedClientPaymentData {
    pub fn payment(&self) -> Any {
        self.inner.get("payment").as_::<Any>()
    }

    pub fn set_payment(&mut self, value: &Any) {
        self.inner.set("payment", value);
    }
}
