use super::*;




/// The PublicKeyCredentialRequestOptionsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialRequestOptionsJSON {
    inner: Any,
}

impl FromVal for PublicKeyCredentialRequestOptionsJSON {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialRequestOptionsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialRequestOptionsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialRequestOptionsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialRequestOptionsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialRequestOptionsJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredentialRequestOptionsJSON> for Any {
    fn from(s: PublicKeyCredentialRequestOptionsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialRequestOptionsJSON> for Any {
    fn from(s: &PublicKeyCredentialRequestOptionsJSON) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `challenge` attribute.
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    /// Setter of the `challenge` attribute.
    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `timeout` attribute.
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `allowCredentials` attribute.
    pub fn allow_credentials(&self) -> TypedArray<PublicKeyCredentialDescriptorJSON> {
        self.inner.get("allowCredentials").as_::<TypedArray<PublicKeyCredentialDescriptorJSON>>()
    }

    /// Setter of the `allowCredentials` attribute.
    pub fn set_allow_credentials(&mut self, value: &TypedArray<PublicKeyCredentialDescriptorJSON>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `userVerification` attribute.
    pub fn user_verification(&self) -> JsString {
        self.inner.get("userVerification").as_::<JsString>()
    }

    /// Setter of the `userVerification` attribute.
    pub fn set_user_verification(&mut self, value: &JsString) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `hints` attribute.
    pub fn hints(&self) -> TypedArray<JsString> {
        self.inner.get("hints").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `hints` attribute.
    pub fn set_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    /// Getter of the `extensions` attribute.
    pub fn extensions(&self) -> AuthenticationExtensionsClientInputsJSON {
        self.inner.get("extensions").as_::<AuthenticationExtensionsClientInputsJSON>()
    }

    /// Setter of the `extensions` attribute.
    pub fn set_extensions(&mut self, value: &AuthenticationExtensionsClientInputsJSON) {
        self.inner.set("extensions", value);
    }
}
