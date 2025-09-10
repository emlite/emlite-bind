use super::*;

/// The PaymentDetailsModifier dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentDetailsModifier {
    inner: Any,
}

impl FromVal for PaymentDetailsModifier {
    fn from_val(v: &Any) -> Self {
        PaymentDetailsModifier { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentDetailsModifier {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentDetailsModifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentDetailsModifier {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentDetailsModifier {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentDetailsModifier> for Any {
    fn from(s: PaymentDetailsModifier) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentDetailsModifier> for Any {
    fn from(s: &PaymentDetailsModifier) -> Any {
        s.inner.clone()
    }
}

impl PaymentDetailsModifier {
    /// Getter of the `supportedMethods` attribute.
    pub fn supported_methods(&self) -> JsString {
        self.inner.get("supportedMethods").as_::<JsString>()
    }

    /// Setter of the `supportedMethods` attribute.
    pub fn set_supported_methods(&mut self, value: &JsString) {
        self.inner.set("supportedMethods", value);
    }
}
impl PaymentDetailsModifier {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentItem {
        self.inner.get("total").as_::<PaymentItem>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentItem) {
        self.inner.set("total", value);
    }
}
impl PaymentDetailsModifier {
    /// Getter of the `additionalDisplayItems` attribute.
    pub fn additional_display_items(&self) -> TypedArray<PaymentItem> {
        self.inner
            .get("additionalDisplayItems")
            .as_::<TypedArray<PaymentItem>>()
    }

    /// Setter of the `additionalDisplayItems` attribute.
    pub fn set_additional_display_items(&mut self, value: &TypedArray<PaymentItem>) {
        self.inner.set("additionalDisplayItems", value);
    }
}
impl PaymentDetailsModifier {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
