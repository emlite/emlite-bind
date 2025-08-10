use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentItem {
    inner: Any,
}
impl FromVal for PaymentItem {
    fn from_val(v: &Any) -> Self {
        PaymentItem { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentItem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentItem> for Any {
    fn from(s: PaymentItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentItem> for Any {
    fn from(s: &PaymentItem) -> Any {
        s.inner.clone()
    }
}

impl PaymentItem {
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl PaymentItem {
    pub fn amount(&self) -> PaymentCurrencyAmount {
        self.inner.get("amount").as_::<PaymentCurrencyAmount>()
    }

    pub fn set_amount(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("amount", value);
    }
}
impl PaymentItem {
    pub fn pending(&self) -> bool {
        self.inner.get("pending").as_::<bool>()
    }

    pub fn set_pending(&mut self, value: bool) {
        self.inner.set("pending", value);
    }
}
