use super::*;




/// The PayerErrors dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PayerErrors {
    inner: Any,
}

impl FromVal for PayerErrors {
    fn from_val(v: &Any) -> Self {
        PayerErrors { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PayerErrors {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PayerErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PayerErrors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PayerErrors {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PayerErrors> for Any {
    fn from(s: PayerErrors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PayerErrors> for Any {
    fn from(s: &PayerErrors) -> Any {
        s.inner.clone()
    }
}

impl PayerErrors {
    /// Getter of the `email` attribute.
    pub fn email(&self) -> JsString {
        self.inner.get("email").as_::<JsString>()
    }

    /// Setter of the `email` attribute.
    pub fn set_email(&mut self, value: &JsString) {
        self.inner.set("email", value);
    }
}
impl PayerErrors {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl PayerErrors {
    /// Getter of the `phone` attribute.
    pub fn phone(&self) -> JsString {
        self.inner.get("phone").as_::<JsString>()
    }

    /// Setter of the `phone` attribute.
    pub fn set_phone(&mut self, value: &JsString) {
        self.inner.set("phone", value);
    }
}
