use super::*;




/// The PublicKeyCredential class.
/// [`PublicKeyCredential`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredential {
    inner: Credential,
}

impl FromVal for PublicKeyCredential {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredential { inner: Credential::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredential {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredential> for Any {
    fn from(s: PublicKeyCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredential> for Any {
    fn from(s: &PublicKeyCredential) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PublicKeyCredential);


impl PublicKeyCredential {
    /// Getter of the `rawId` attribute.
    /// [`PublicKeyCredential.rawId`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)
    pub fn raw_id(&self) -> ArrayBuffer {
        self.inner.get("rawId").as_::<ArrayBuffer>()
    }

}
impl PublicKeyCredential {
    /// Getter of the `response` attribute.
    /// [`PublicKeyCredential.response`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)
    pub fn response(&self) -> AuthenticatorResponse {
        self.inner.get("response").as_::<AuthenticatorResponse>()
    }

}
impl PublicKeyCredential {
    /// Getter of the `authenticatorAttachment` attribute.
    /// [`PublicKeyCredential.authenticatorAttachment`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/authenticatorAttachment)
    pub fn authenticator_attachment(&self) -> JsString {
        self.inner.get("authenticatorAttachment").as_::<JsString>()
    }

}
impl PublicKeyCredential {
    /// The getClientExtensionResults method.
    /// [`PublicKeyCredential.getClientExtensionResults`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)
    pub fn get_client_extension_results(&self, ) -> AuthenticationExtensionsClientOutputs {
        self.inner.call("getClientExtensionResults", &[]).as_::<AuthenticationExtensionsClientOutputs>()
    }
}
impl PublicKeyCredential {
    /// The isConditionalMediationAvailable method.
    /// [`PublicKeyCredential.isConditionalMediationAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isConditionalMediationAvailable)
    pub fn is_conditional_mediation_available() -> Promise<bool> {
        Any::global("PublicKeyCredential").call("isConditionalMediationAvailable", &[]).as_::<Promise<bool>>()
    }
}
impl PublicKeyCredential {
    /// The toJSON method.
    /// [`PublicKeyCredential.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/toJSON)
    pub fn to_json(&self, ) -> Any {
        self.inner.call("toJSON", &[]).as_::<Any>()
    }
}
impl PublicKeyCredential {
    /// The isUserVerifyingPlatformAuthenticatorAvailable method.
    /// [`PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)
    pub fn is_user_verifying_platform_authenticator_available() -> Promise<bool> {
        Any::global("PublicKeyCredential").call("isUserVerifyingPlatformAuthenticatorAvailable", &[]).as_::<Promise<bool>>()
    }
}
impl PublicKeyCredential {
    /// The getClientCapabilities method.
    /// [`PublicKeyCredential.getClientCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientCapabilities)
    pub fn get_client_capabilities() -> Promise<Any> {
        Any::global("PublicKeyCredential").call("getClientCapabilities", &[]).as_::<Promise<Any>>()
    }
}
impl PublicKeyCredential {
    /// The parseCreationOptionsFromJSON method.
    /// [`PublicKeyCredential.parseCreationOptionsFromJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseCreationOptionsFromJSON)
    pub fn parse_creation_options_from_json(options: &PublicKeyCredentialCreationOptionsJSON) -> PublicKeyCredentialCreationOptions {
        Any::global("PublicKeyCredential").call("parseCreationOptionsFromJSON", &[options.into(), ]).as_::<PublicKeyCredentialCreationOptions>()
    }
}
impl PublicKeyCredential {
    /// The parseRequestOptionsFromJSON method.
    /// [`PublicKeyCredential.parseRequestOptionsFromJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseRequestOptionsFromJSON)
    pub fn parse_request_options_from_json(options: &PublicKeyCredentialRequestOptionsJSON) -> PublicKeyCredentialRequestOptions {
        Any::global("PublicKeyCredential").call("parseRequestOptionsFromJSON", &[options.into(), ]).as_::<PublicKeyCredentialRequestOptions>()
    }
}
impl PublicKeyCredential {
    /// The signalUnknownCredential method.
    /// [`PublicKeyCredential.signalUnknownCredential`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalUnknownCredential)
    pub fn signal_unknown_credential(options: &UnknownCredentialOptions) -> Promise<Undefined> {
        Any::global("PublicKeyCredential").call("signalUnknownCredential", &[options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl PublicKeyCredential {
    /// The signalAllAcceptedCredentials method.
    /// [`PublicKeyCredential.signalAllAcceptedCredentials`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalAllAcceptedCredentials)
    pub fn signal_all_accepted_credentials(options: &AllAcceptedCredentialsOptions) -> Promise<Undefined> {
        Any::global("PublicKeyCredential").call("signalAllAcceptedCredentials", &[options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl PublicKeyCredential {
    /// The signalCurrentUserDetails method.
    /// [`PublicKeyCredential.signalCurrentUserDetails`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalCurrentUserDetails)
    pub fn signal_current_user_details(options: &CurrentUserDetailsOptions) -> Promise<Undefined> {
        Any::global("PublicKeyCredential").call("signalCurrentUserDetails", &[options.into(), ]).as_::<Promise<Undefined>>()
    }
}
