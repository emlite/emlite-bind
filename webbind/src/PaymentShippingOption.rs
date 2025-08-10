use super::*;

/// The PaymentShippingOption dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentShippingOption {
    inner: Any,
}

impl FromVal for PaymentShippingOption {
    fn from_val(v: &Any) -> Self {
        PaymentShippingOption { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentShippingOption {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentShippingOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentShippingOption {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentShippingOption {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentShippingOption> for Any {
    fn from(s: PaymentShippingOption) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentShippingOption> for Any {
    fn from(s: &PaymentShippingOption) -> Any {
        s.inner.clone()
    }
}

impl PaymentShippingOption {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl PaymentShippingOption {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl PaymentShippingOption {
    /// Getter of the `amount` attribute.
    pub fn amount(&self) -> PaymentCurrencyAmount {
        self.inner.get("amount").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `amount` attribute.
    pub fn set_amount(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("amount", value);
    }
}
impl PaymentShippingOption {
    /// Getter of the `selected` attribute.
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    /// Setter of the `selected` attribute.
    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
