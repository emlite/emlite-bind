use super::*;




/// The PaymentDetailsInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentDetailsInit {
    inner: Any,
}

impl FromVal for PaymentDetailsInit {
    fn from_val(v: &Any) -> Self {
        PaymentDetailsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentDetailsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentDetailsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentDetailsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentDetailsInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentDetailsInit> for Any {
    fn from(s: PaymentDetailsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentDetailsInit> for Any {
    fn from(s: &PaymentDetailsInit) -> Any {
        s.inner.clone()
    }
}

impl PaymentDetailsInit {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl PaymentDetailsInit {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentItem {
        self.inner.get("total").as_::<PaymentItem>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentItem) {
        self.inner.set("total", value);
    }
}
