use super::*;

/// The PublicKeyCredentialRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialRequestOptions {
    inner: Any,
}

impl FromVal for PublicKeyCredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PublicKeyCredentialRequestOptions> for Any {
    fn from(s: PublicKeyCredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialRequestOptions> for Any {
    fn from(s: &PublicKeyCredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialRequestOptions {
    /// Getter of the `challenge` attribute.
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    /// Setter of the `challenge` attribute.
    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `timeout` attribute.
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `allowCredentials` attribute.
    pub fn allow_credentials(&self) -> TypedArray<PublicKeyCredentialDescriptor> {
        self.inner
            .get("allowCredentials")
            .as_::<TypedArray<PublicKeyCredentialDescriptor>>()
    }

    /// Setter of the `allowCredentials` attribute.
    pub fn set_allow_credentials(&mut self, value: &TypedArray<PublicKeyCredentialDescriptor>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `userVerification` attribute.
    pub fn user_verification(&self) -> JsString {
        self.inner.get("userVerification").as_::<JsString>()
    }

    /// Setter of the `userVerification` attribute.
    pub fn set_user_verification(&mut self, value: &JsString) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `hints` attribute.
    pub fn hints(&self) -> TypedArray<JsString> {
        self.inner.get("hints").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `hints` attribute.
    pub fn set_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    /// Getter of the `extensions` attribute.
    pub fn extensions(&self) -> AuthenticationExtensionsClientInputs {
        self.inner
            .get("extensions")
            .as_::<AuthenticationExtensionsClientInputs>()
    }

    /// Setter of the `extensions` attribute.
    pub fn set_extensions(&mut self, value: &AuthenticationExtensionsClientInputs) {
        self.inner.set("extensions", value);
    }
}
