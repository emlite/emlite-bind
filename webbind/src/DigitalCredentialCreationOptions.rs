use super::*;

/// The DigitalCredentialCreationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalCredentialCreationOptions {
    inner: Any,
}

impl FromVal for DigitalCredentialCreationOptions {
    fn from_val(v: &Any) -> Self {
        DigitalCredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DigitalCredentialCreationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DigitalCredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DigitalCredentialCreationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DigitalCredentialCreationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DigitalCredentialCreationOptions> for Any {
    fn from(s: DigitalCredentialCreationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DigitalCredentialCreationOptions> for Any {
    fn from(s: &DigitalCredentialCreationOptions) -> Any {
        s.inner.clone()
    }
}

impl DigitalCredentialCreationOptions {
    /// Getter of the `requests` attribute.
    pub fn requests(&self) -> TypedArray<DigitalCredentialCreateRequest> {
        self.inner
            .get("requests")
            .as_::<TypedArray<DigitalCredentialCreateRequest>>()
    }

    /// Setter of the `requests` attribute.
    pub fn set_requests(&mut self, value: &TypedArray<DigitalCredentialCreateRequest>) {
        self.inner.set("requests", value);
    }
}
