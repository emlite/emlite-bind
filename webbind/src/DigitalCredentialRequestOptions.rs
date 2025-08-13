use super::*;




/// The DigitalCredentialRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalCredentialRequestOptions {
    inner: Any,
}

impl FromVal for DigitalCredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        DigitalCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DigitalCredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DigitalCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DigitalCredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DigitalCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DigitalCredentialRequestOptions> for Any {
    fn from(s: DigitalCredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DigitalCredentialRequestOptions> for Any {
    fn from(s: &DigitalCredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl DigitalCredentialRequestOptions {
    /// Getter of the `requests` attribute.
    pub fn requests(&self) -> TypedArray<DigitalCredentialGetRequest> {
        self.inner.get("requests").as_::<TypedArray<DigitalCredentialGetRequest>>()
    }

    /// Setter of the `requests` attribute.
    pub fn set_requests(&mut self, value: &TypedArray<DigitalCredentialGetRequest>) {
        self.inner.set("requests", value);
    }
}
