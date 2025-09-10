use super::*;

/// The DigitalCredentialGetRequest dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalCredentialGetRequest {
    inner: Any,
}

impl FromVal for DigitalCredentialGetRequest {
    fn from_val(v: &Any) -> Self {
        DigitalCredentialGetRequest { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DigitalCredentialGetRequest {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DigitalCredentialGetRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DigitalCredentialGetRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DigitalCredentialGetRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DigitalCredentialGetRequest> for Any {
    fn from(s: DigitalCredentialGetRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DigitalCredentialGetRequest> for Any {
    fn from(s: &DigitalCredentialGetRequest) -> Any {
        s.inner.clone()
    }
}

impl DigitalCredentialGetRequest {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl DigitalCredentialGetRequest {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
