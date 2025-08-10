use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentCredentialInstrument {
    inner: Any,
}
impl FromVal for PaymentCredentialInstrument {
    fn from_val(v: &Any) -> Self {
        PaymentCredentialInstrument { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentCredentialInstrument {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentCredentialInstrument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentCredentialInstrument {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentCredentialInstrument {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentCredentialInstrument> for Any {
    fn from(s: PaymentCredentialInstrument) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentCredentialInstrument> for Any {
    fn from(s: &PaymentCredentialInstrument) -> Any {
        s.inner.clone()
    }
}

impl PaymentCredentialInstrument {
    pub fn display_name(&self) -> JsString {
        self.inner.get("displayName").as_::<JsString>()
    }

    pub fn set_display_name(&mut self, value: &JsString) {
        self.inner.set("displayName", value);
    }
}
impl PaymentCredentialInstrument {
    pub fn icon(&self) -> JsString {
        self.inner.get("icon").as_::<JsString>()
    }

    pub fn set_icon(&mut self, value: &JsString) {
        self.inner.set("icon", value);
    }
}
impl PaymentCredentialInstrument {
    pub fn icon_must_be_shown(&self) -> bool {
        self.inner.get("iconMustBeShown").as_::<bool>()
    }

    pub fn set_icon_must_be_shown(&mut self, value: bool) {
        self.inner.set("iconMustBeShown", value);
    }
}
impl PaymentCredentialInstrument {
    pub fn details(&self) -> JsString {
        self.inner.get("details").as_::<JsString>()
    }

    pub fn set_details(&mut self, value: &JsString) {
        self.inner.set("details", value);
    }
}
