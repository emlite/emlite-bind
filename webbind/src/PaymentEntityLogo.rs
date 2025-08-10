use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentEntityLogo {
    inner: Any,
}
impl FromVal for PaymentEntityLogo {
    fn from_val(v: &Any) -> Self {
        PaymentEntityLogo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentEntityLogo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentEntityLogo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentEntityLogo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentEntityLogo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentEntityLogo> for Any {
    fn from(s: PaymentEntityLogo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentEntityLogo> for Any {
    fn from(s: &PaymentEntityLogo) -> Any {
        s.inner.clone()
    }
}

impl PaymentEntityLogo {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl PaymentEntityLogo {
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
