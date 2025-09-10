use super::*;

/// The PaymentOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentOptions {
    inner: Any,
}

impl FromVal for PaymentOptions {
    fn from_val(v: &Any) -> Self {
        PaymentOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentOptions> for Any {
    fn from(s: PaymentOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentOptions> for Any {
    fn from(s: &PaymentOptions) -> Any {
        s.inner.clone()
    }
}

impl PaymentOptions {
    /// Getter of the `requestPayerName` attribute.
    pub fn request_payer_name(&self) -> bool {
        self.inner.get("requestPayerName").as_::<bool>()
    }

    /// Setter of the `requestPayerName` attribute.
    pub fn set_request_payer_name(&mut self, value: bool) {
        self.inner.set("requestPayerName", value);
    }
}
impl PaymentOptions {
    /// Getter of the `requestBillingAddress` attribute.
    pub fn request_billing_address(&self) -> bool {
        self.inner.get("requestBillingAddress").as_::<bool>()
    }

    /// Setter of the `requestBillingAddress` attribute.
    pub fn set_request_billing_address(&mut self, value: bool) {
        self.inner.set("requestBillingAddress", value);
    }
}
impl PaymentOptions {
    /// Getter of the `requestPayerEmail` attribute.
    pub fn request_payer_email(&self) -> bool {
        self.inner.get("requestPayerEmail").as_::<bool>()
    }

    /// Setter of the `requestPayerEmail` attribute.
    pub fn set_request_payer_email(&mut self, value: bool) {
        self.inner.set("requestPayerEmail", value);
    }
}
impl PaymentOptions {
    /// Getter of the `requestPayerPhone` attribute.
    pub fn request_payer_phone(&self) -> bool {
        self.inner.get("requestPayerPhone").as_::<bool>()
    }

    /// Setter of the `requestPayerPhone` attribute.
    pub fn set_request_payer_phone(&mut self, value: bool) {
        self.inner.set("requestPayerPhone", value);
    }
}
impl PaymentOptions {
    /// Getter of the `requestShipping` attribute.
    pub fn request_shipping(&self) -> bool {
        self.inner.get("requestShipping").as_::<bool>()
    }

    /// Setter of the `requestShipping` attribute.
    pub fn set_request_shipping(&mut self, value: bool) {
        self.inner.set("requestShipping", value);
    }
}
impl PaymentOptions {
    /// Getter of the `shippingType` attribute.
    pub fn shipping_type(&self) -> PaymentShippingType {
        self.inner.get("shippingType").as_::<PaymentShippingType>()
    }

    /// Setter of the `shippingType` attribute.
    pub fn set_shipping_type(&mut self, value: &PaymentShippingType) {
        self.inner.set("shippingType", value);
    }
}
