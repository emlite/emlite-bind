use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentMethodChangeEventInit {
    inner: Any,
}
impl FromVal for PaymentMethodChangeEventInit {
    fn from_val(v: &Any) -> Self {
        PaymentMethodChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentMethodChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentMethodChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentMethodChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentMethodChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentMethodChangeEventInit> for Any {
    fn from(s: PaymentMethodChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentMethodChangeEventInit> for Any {
    fn from(s: &PaymentMethodChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl PaymentMethodChangeEventInit {
    pub fn method_name(&self) -> JsString {
        self.inner.get("methodName").as_::<JsString>()
    }

    pub fn set_method_name(&mut self, value: &JsString) {
        self.inner.set("methodName", value);
    }
}
impl PaymentMethodChangeEventInit {
    pub fn method_details(&self) -> Object {
        self.inner.get("methodDetails").as_::<Object>()
    }

    pub fn set_method_details(&mut self, value: &Object) {
        self.inner.set("methodDetails", value);
    }
}
