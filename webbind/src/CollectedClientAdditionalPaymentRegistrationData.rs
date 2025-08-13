use super::*;




/// The CollectedClientAdditionalPaymentRegistrationData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CollectedClientAdditionalPaymentRegistrationData {
    inner: Any,
}

impl FromVal for CollectedClientAdditionalPaymentRegistrationData {
    fn from_val(v: &Any) -> Self {
        CollectedClientAdditionalPaymentRegistrationData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CollectedClientAdditionalPaymentRegistrationData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CollectedClientAdditionalPaymentRegistrationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CollectedClientAdditionalPaymentRegistrationData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CollectedClientAdditionalPaymentRegistrationData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CollectedClientAdditionalPaymentRegistrationData> for Any {
    fn from(s: CollectedClientAdditionalPaymentRegistrationData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CollectedClientAdditionalPaymentRegistrationData> for Any {
    fn from(s: &CollectedClientAdditionalPaymentRegistrationData) -> Any {
        s.inner.clone()
    }
}

impl CollectedClientAdditionalPaymentRegistrationData {
    /// Getter of the `browserBoundPublicKey` attribute.
    pub fn browser_bound_public_key(&self) -> JsString {
        self.inner.get("browserBoundPublicKey").as_::<JsString>()
    }

    /// Setter of the `browserBoundPublicKey` attribute.
    pub fn set_browser_bound_public_key(&mut self, value: &JsString) {
        self.inner.set("browserBoundPublicKey", value);
    }
}
