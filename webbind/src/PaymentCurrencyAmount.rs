use super::*;




/// The PaymentCurrencyAmount dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentCurrencyAmount {
    inner: Any,
}

impl FromVal for PaymentCurrencyAmount {
    fn from_val(v: &Any) -> Self {
        PaymentCurrencyAmount { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentCurrencyAmount {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentCurrencyAmount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentCurrencyAmount {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentCurrencyAmount {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentCurrencyAmount> for Any {
    fn from(s: PaymentCurrencyAmount) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentCurrencyAmount> for Any {
    fn from(s: &PaymentCurrencyAmount) -> Any {
        s.inner.clone()
    }
}

impl PaymentCurrencyAmount {
    /// Getter of the `currency` attribute.
    pub fn currency(&self) -> JsString {
        self.inner.get("currency").as_::<JsString>()
    }

    /// Setter of the `currency` attribute.
    pub fn set_currency(&mut self, value: &JsString) {
        self.inner.set("currency", value);
    }
}
impl PaymentCurrencyAmount {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
