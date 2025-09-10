use super::*;

/// The PublicKeyCredentialCreationOptionsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialCreationOptionsJSON {
    inner: Any,
}

impl FromVal for PublicKeyCredentialCreationOptionsJSON {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialCreationOptionsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialCreationOptionsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialCreationOptionsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialCreationOptionsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialCreationOptionsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PublicKeyCredentialCreationOptionsJSON> for Any {
    fn from(s: PublicKeyCredentialCreationOptionsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialCreationOptionsJSON> for Any {
    fn from(s: &PublicKeyCredentialCreationOptionsJSON) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `rp` attribute.
    pub fn rp(&self) -> PublicKeyCredentialRpEntity {
        self.inner.get("rp").as_::<PublicKeyCredentialRpEntity>()
    }

    /// Setter of the `rp` attribute.
    pub fn set_rp(&mut self, value: &PublicKeyCredentialRpEntity) {
        self.inner.set("rp", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `user` attribute.
    pub fn user(&self) -> PublicKeyCredentialUserEntityJSON {
        self.inner
            .get("user")
            .as_::<PublicKeyCredentialUserEntityJSON>()
    }

    /// Setter of the `user` attribute.
    pub fn set_user(&mut self, value: &PublicKeyCredentialUserEntityJSON) {
        self.inner.set("user", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `challenge` attribute.
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    /// Setter of the `challenge` attribute.
    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `pubKeyCredParams` attribute.
    pub fn pub_key_cred_params(&self) -> TypedArray<PublicKeyCredentialParameters> {
        self.inner
            .get("pubKeyCredParams")
            .as_::<TypedArray<PublicKeyCredentialParameters>>()
    }

    /// Setter of the `pubKeyCredParams` attribute.
    pub fn set_pub_key_cred_params(&mut self, value: &TypedArray<PublicKeyCredentialParameters>) {
        self.inner.set("pubKeyCredParams", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `timeout` attribute.
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `excludeCredentials` attribute.
    pub fn exclude_credentials(&self) -> TypedArray<PublicKeyCredentialDescriptorJSON> {
        self.inner
            .get("excludeCredentials")
            .as_::<TypedArray<PublicKeyCredentialDescriptorJSON>>()
    }

    /// Setter of the `excludeCredentials` attribute.
    pub fn set_exclude_credentials(
        &mut self,
        value: &TypedArray<PublicKeyCredentialDescriptorJSON>,
    ) {
        self.inner.set("excludeCredentials", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `authenticatorSelection` attribute.
    pub fn authenticator_selection(&self) -> AuthenticatorSelectionCriteria {
        self.inner
            .get("authenticatorSelection")
            .as_::<AuthenticatorSelectionCriteria>()
    }

    /// Setter of the `authenticatorSelection` attribute.
    pub fn set_authenticator_selection(&mut self, value: &AuthenticatorSelectionCriteria) {
        self.inner.set("authenticatorSelection", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `hints` attribute.
    pub fn hints(&self) -> TypedArray<JsString> {
        self.inner.get("hints").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `hints` attribute.
    pub fn set_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `attestation` attribute.
    pub fn attestation(&self) -> JsString {
        self.inner.get("attestation").as_::<JsString>()
    }

    /// Setter of the `attestation` attribute.
    pub fn set_attestation(&mut self, value: &JsString) {
        self.inner.set("attestation", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `attestationFormats` attribute.
    pub fn attestation_formats(&self) -> TypedArray<JsString> {
        self.inner
            .get("attestationFormats")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `attestationFormats` attribute.
    pub fn set_attestation_formats(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("attestationFormats", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    /// Getter of the `extensions` attribute.
    pub fn extensions(&self) -> AuthenticationExtensionsClientInputsJSON {
        self.inner
            .get("extensions")
            .as_::<AuthenticationExtensionsClientInputsJSON>()
    }

    /// Setter of the `extensions` attribute.
    pub fn set_extensions(&mut self, value: &AuthenticationExtensionsClientInputsJSON) {
        self.inner.set("extensions", value);
    }
}
