use super::*;




/// The PaymentMethodData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentMethodData {
    inner: Any,
}

impl FromVal for PaymentMethodData {
    fn from_val(v: &Any) -> Self {
        PaymentMethodData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentMethodData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentMethodData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentMethodData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentMethodData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentMethodData> for Any {
    fn from(s: PaymentMethodData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentMethodData> for Any {
    fn from(s: &PaymentMethodData) -> Any {
        s.inner.clone()
    }
}

impl PaymentMethodData {
    /// Getter of the `supportedMethods` attribute.
    pub fn supported_methods(&self) -> JsString {
        self.inner.get("supportedMethods").as_::<JsString>()
    }

    /// Setter of the `supportedMethods` attribute.
    pub fn set_supported_methods(&mut self, value: &JsString) {
        self.inner.set("supportedMethods", value);
    }
}
impl PaymentMethodData {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
