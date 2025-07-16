use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsClientOutputs {
    inner: Any,
}
impl FromVal for AuthenticationExtensionsClientOutputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsClientOutputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticationExtensionsClientOutputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticationExtensionsClientOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticationExtensionsClientOutputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticationExtensionsClientOutputs {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticationExtensionsClientOutputs> for Any {
    fn from(s: AuthenticationExtensionsClientOutputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticationExtensionsClientOutputs> for Any {
    fn from(s: &AuthenticationExtensionsClientOutputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsClientOutputs {
    pub fn large_blob(&self) -> Any {
        self.inner.get("largeBlob").as_::<Any>()
    }

    pub fn set_large_blob(&mut self, value: &Any) {
        self.inner.set("largeBlob", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialCreationOptions {
    inner: Any,
}
impl FromVal for PublicKeyCredentialCreationOptions {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PublicKeyCredentialCreationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PublicKeyCredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PublicKeyCredentialCreationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PublicKeyCredentialCreationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialCreationOptions> for Any {
    fn from(s: PublicKeyCredentialCreationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PublicKeyCredentialCreationOptions> for Any {
    fn from(s: &PublicKeyCredentialCreationOptions) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialCreationOptions {
    pub fn rp(&self) -> Any {
        self.inner.get("rp").as_::<Any>()
    }

    pub fn set_rp(&mut self, value: &Any) {
        self.inner.set("rp", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn user(&self) -> Any {
        self.inner.get("user").as_::<Any>()
    }

    pub fn set_user(&mut self, value: &Any) {
        self.inner.set("user", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn pub_key_cred_params(&self) -> Sequence<Any> {
        self.inner.get("pubKeyCredParams").as_::<Sequence<Any>>()
    }

    pub fn set_pub_key_cred_params(&mut self, value: &Sequence<Any>) {
        self.inner.set("pubKeyCredParams", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn exclude_credentials(&self) -> Sequence<Any> {
        self.inner.get("excludeCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_exclude_credentials(&mut self, value: &Sequence<Any>) {
        self.inner.set("excludeCredentials", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn authenticator_selection(&self) -> Any {
        self.inner.get("authenticatorSelection").as_::<Any>()
    }

    pub fn set_authenticator_selection(&mut self, value: &Any) {
        self.inner.set("authenticatorSelection", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn hints(&self) -> Sequence<String> {
        self.inner.get("hints").as_::<Sequence<String>>()
    }

    pub fn set_hints(&mut self, value: &Sequence<String>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn attestation(&self) -> String {
        self.inner.get("attestation").as_::<String>()
    }

    pub fn set_attestation(&mut self, value: &str) {
        self.inner.set("attestation", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn attestation_formats(&self) -> Sequence<String> {
        self.inner
            .get("attestationFormats")
            .as_::<Sequence<String>>()
    }

    pub fn set_attestation_formats(&mut self, value: &Sequence<String>) {
        self.inner.set("attestationFormats", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: &Any) {
        self.inner.set("extensions", value);
    }
}
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
    pub fn rp(&self) -> Any {
        self.inner.get("rp").as_::<Any>()
    }

    pub fn set_rp(&mut self, value: &Any) {
        self.inner.set("rp", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn user(&self) -> Any {
        self.inner.get("user").as_::<Any>()
    }

    pub fn set_user(&mut self, value: &Any) {
        self.inner.set("user", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn pub_key_cred_params(&self) -> Sequence<Any> {
        self.inner.get("pubKeyCredParams").as_::<Sequence<Any>>()
    }

    pub fn set_pub_key_cred_params(&mut self, value: &Sequence<Any>) {
        self.inner.set("pubKeyCredParams", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn exclude_credentials(&self) -> Sequence<Any> {
        self.inner.get("excludeCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_exclude_credentials(&mut self, value: &Sequence<Any>) {
        self.inner.set("excludeCredentials", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn authenticator_selection(&self) -> Any {
        self.inner.get("authenticatorSelection").as_::<Any>()
    }

    pub fn set_authenticator_selection(&mut self, value: &Any) {
        self.inner.set("authenticatorSelection", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn hints(&self) -> Sequence<String> {
        self.inner.get("hints").as_::<Sequence<String>>()
    }

    pub fn set_hints(&mut self, value: &Sequence<String>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn attestation(&self) -> String {
        self.inner.get("attestation").as_::<String>()
    }

    pub fn set_attestation(&mut self, value: &str) {
        self.inner.set("attestation", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn attestation_formats(&self) -> Sequence<String> {
        self.inner
            .get("attestationFormats")
            .as_::<Sequence<String>>()
    }

    pub fn set_attestation_formats(&mut self, value: &Sequence<String>) {
        self.inner.set("attestationFormats", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: &Any) {
        self.inner.set("extensions", value);
    }
}
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
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn rp_id(&self) -> String {
        self.inner.get("rpId").as_::<String>()
    }

    pub fn set_rp_id(&mut self, value: &str) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn allow_credentials(&self) -> Sequence<Any> {
        self.inner.get("allowCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_allow_credentials(&mut self, value: &Sequence<Any>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn user_verification(&self) -> String {
        self.inner.get("userVerification").as_::<String>()
    }

    pub fn set_user_verification(&mut self, value: &str) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn hints(&self) -> Sequence<String> {
        self.inner.get("hints").as_::<Sequence<String>>()
    }

    pub fn set_hints(&mut self, value: &Sequence<String>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: &Any) {
        self.inner.set("extensions", value);
    }
}
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
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn rp_id(&self) -> String {
        self.inner.get("rpId").as_::<String>()
    }

    pub fn set_rp_id(&mut self, value: &str) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn allow_credentials(&self) -> Sequence<Any> {
        self.inner.get("allowCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_allow_credentials(&mut self, value: &Sequence<Any>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn user_verification(&self) -> String {
        self.inner.get("userVerification").as_::<String>()
    }

    pub fn set_user_verification(&mut self, value: &str) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn hints(&self) -> Sequence<String> {
        self.inner.get("hints").as_::<Sequence<String>>()
    }

    pub fn set_hints(&mut self, value: &Sequence<String>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: &Any) {
        self.inner.set("extensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnknownCredentialOptions {
    inner: Any,
}
impl FromVal for UnknownCredentialOptions {
    fn from_val(v: &Any) -> Self {
        UnknownCredentialOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for UnknownCredentialOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UnknownCredentialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UnknownCredentialOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UnknownCredentialOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UnknownCredentialOptions> for Any {
    fn from(s: UnknownCredentialOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UnknownCredentialOptions> for Any {
    fn from(s: &UnknownCredentialOptions) -> Any {
        s.inner.clone()
    }
}

impl UnknownCredentialOptions {
    pub fn rp_id(&self) -> String {
        self.inner.get("rpId").as_::<String>()
    }

    pub fn set_rp_id(&mut self, value: &str) {
        self.inner.set("rpId", value);
    }
}
impl UnknownCredentialOptions {
    pub fn credential_id(&self) -> Any {
        self.inner.get("credentialId").as_::<Any>()
    }

    pub fn set_credential_id(&mut self, value: &Any) {
        self.inner.set("credentialId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AllAcceptedCredentialsOptions {
    inner: Any,
}
impl FromVal for AllAcceptedCredentialsOptions {
    fn from_val(v: &Any) -> Self {
        AllAcceptedCredentialsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AllAcceptedCredentialsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AllAcceptedCredentialsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AllAcceptedCredentialsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AllAcceptedCredentialsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AllAcceptedCredentialsOptions> for Any {
    fn from(s: AllAcceptedCredentialsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AllAcceptedCredentialsOptions> for Any {
    fn from(s: &AllAcceptedCredentialsOptions) -> Any {
        s.inner.clone()
    }
}

impl AllAcceptedCredentialsOptions {
    pub fn rp_id(&self) -> String {
        self.inner.get("rpId").as_::<String>()
    }

    pub fn set_rp_id(&mut self, value: &str) {
        self.inner.set("rpId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    pub fn set_user_id(&mut self, value: &Any) {
        self.inner.set("userId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    pub fn all_accepted_credential_ids(&self) -> Sequence<Any> {
        self.inner
            .get("allAcceptedCredentialIds")
            .as_::<Sequence<Any>>()
    }

    pub fn set_all_accepted_credential_ids(&mut self, value: &Sequence<Any>) {
        self.inner.set("allAcceptedCredentialIds", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CurrentUserDetailsOptions {
    inner: Any,
}
impl FromVal for CurrentUserDetailsOptions {
    fn from_val(v: &Any) -> Self {
        CurrentUserDetailsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CurrentUserDetailsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CurrentUserDetailsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CurrentUserDetailsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CurrentUserDetailsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CurrentUserDetailsOptions> for Any {
    fn from(s: CurrentUserDetailsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CurrentUserDetailsOptions> for Any {
    fn from(s: &CurrentUserDetailsOptions) -> Any {
        s.inner.clone()
    }
}

impl CurrentUserDetailsOptions {
    pub fn rp_id(&self) -> String {
        self.inner.get("rpId").as_::<String>()
    }

    pub fn set_rp_id(&mut self, value: &str) {
        self.inner.set("rpId", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    pub fn set_user_id(&mut self, value: &Any) {
        self.inner.set("userId", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn display_name(&self) -> String {
        self.inner.get("displayName").as_::<String>()
    }

    pub fn set_display_name(&mut self, value: &str) {
        self.inner.set("displayName", value);
    }
}
/// The PublicKeyCredential class.
/// [`PublicKeyCredential`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredential {
    inner: Credential,
}
impl FromVal for PublicKeyCredential {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredential {
            inner: Credential::from_val(v),
        }
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
    pub fn authenticator_attachment(&self) -> String {
        self.inner.get("authenticatorAttachment").as_::<String>()
    }
}
impl PublicKeyCredential {
    /// The getClientExtensionResults method.
    /// [`PublicKeyCredential.getClientExtensionResults`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)
    pub fn get_client_extension_results(&self) -> AuthenticationExtensionsClientOutputs {
        self.inner
            .call("getClientExtensionResults", &[])
            .as_::<AuthenticationExtensionsClientOutputs>()
    }
}
impl PublicKeyCredential {
    /// The isConditionalMediationAvailable method.
    /// [`PublicKeyCredential.isConditionalMediationAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isConditionalMediationAvailable)
    pub fn is_conditional_mediation_available() -> Promise {
        Any::global("PublicKeyCredential")
            .call("isConditionalMediationAvailable", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    /// The toJSON method.
    /// [`PublicKeyCredential.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/toJSON)
    pub fn to_json(&self) -> Any {
        self.inner.call("toJSON", &[]).as_::<Any>()
    }
}
impl PublicKeyCredential {
    /// The isUserVerifyingPlatformAuthenticatorAvailable method.
    /// [`PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)
    pub fn is_user_verifying_platform_authenticator_available() -> Promise {
        Any::global("PublicKeyCredential")
            .call("isUserVerifyingPlatformAuthenticatorAvailable", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    /// The getClientCapabilities method.
    /// [`PublicKeyCredential.getClientCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientCapabilities)
    pub fn get_client_capabilities() -> Promise {
        Any::global("PublicKeyCredential")
            .call("getClientCapabilities", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    /// The parseCreationOptionsFromJSON method.
    /// [`PublicKeyCredential.parseCreationOptionsFromJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseCreationOptionsFromJSON)
    pub fn parse_creation_options_from_json(
        options: &PublicKeyCredentialCreationOptionsJSON,
    ) -> PublicKeyCredentialCreationOptions {
        Any::global("PublicKeyCredential")
            .call("parseCreationOptionsFromJSON", &[options.into()])
            .as_::<PublicKeyCredentialCreationOptions>()
    }
}
impl PublicKeyCredential {
    /// The parseRequestOptionsFromJSON method.
    /// [`PublicKeyCredential.parseRequestOptionsFromJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseRequestOptionsFromJSON)
    pub fn parse_request_options_from_json(
        options: &PublicKeyCredentialRequestOptionsJSON,
    ) -> PublicKeyCredentialRequestOptions {
        Any::global("PublicKeyCredential")
            .call("parseRequestOptionsFromJSON", &[options.into()])
            .as_::<PublicKeyCredentialRequestOptions>()
    }
}
impl PublicKeyCredential {
    /// The signalUnknownCredential method.
    /// [`PublicKeyCredential.signalUnknownCredential`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalUnknownCredential)
    pub fn signal_unknown_credential(options: &UnknownCredentialOptions) -> Promise {
        Any::global("PublicKeyCredential")
            .call("signalUnknownCredential", &[options.into()])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    /// The signalAllAcceptedCredentials method.
    /// [`PublicKeyCredential.signalAllAcceptedCredentials`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalAllAcceptedCredentials)
    pub fn signal_all_accepted_credentials(options: &AllAcceptedCredentialsOptions) -> Promise {
        Any::global("PublicKeyCredential")
            .call("signalAllAcceptedCredentials", &[options.into()])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    /// The signalCurrentUserDetails method.
    /// [`PublicKeyCredential.signalCurrentUserDetails`](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/signalCurrentUserDetails)
    pub fn signal_current_user_details(options: &CurrentUserDetailsOptions) -> Promise {
        Any::global("PublicKeyCredential")
            .call("signalCurrentUserDetails", &[options.into()])
            .as_::<Promise>()
    }
}
