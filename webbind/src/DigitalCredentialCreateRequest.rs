use super::*;




/// The DigitalCredentialCreateRequest dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalCredentialCreateRequest {
    inner: Any,
}

impl FromVal for DigitalCredentialCreateRequest {
    fn from_val(v: &Any) -> Self {
        DigitalCredentialCreateRequest { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DigitalCredentialCreateRequest {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DigitalCredentialCreateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DigitalCredentialCreateRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DigitalCredentialCreateRequest {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DigitalCredentialCreateRequest> for Any {
    fn from(s: DigitalCredentialCreateRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DigitalCredentialCreateRequest> for Any {
    fn from(s: &DigitalCredentialCreateRequest) -> Any {
        s.inner.clone()
    }
}

impl DigitalCredentialCreateRequest {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl DigitalCredentialCreateRequest {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
