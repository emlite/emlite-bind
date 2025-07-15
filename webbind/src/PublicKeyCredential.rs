use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsClientOutputs {
    inner: emlite::Val,
}
impl FromVal for AuthenticationExtensionsClientOutputs {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticationExtensionsClientOutputs { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticationExtensionsClientOutputs {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticationExtensionsClientOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AuthenticationExtensionsClientOutputs {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AuthenticationExtensionsClientOutputs {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AuthenticationExtensionsClientOutputs> for emlite::Val {
    fn from(s: AuthenticationExtensionsClientOutputs) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AuthenticationExtensionsClientOutputs {
    pub fn large_blob(&self) -> Any {
        self.inner.get("largeBlob").as_::<Any>()
    }

    pub fn set_large_blob(&mut self, value: Any) {
        self.inner.set("largeBlob", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialCreationOptions {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialCreationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PublicKeyCredentialCreationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PublicKeyCredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PublicKeyCredentialCreationOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PublicKeyCredentialCreationOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialCreationOptions> for emlite::Val {
    fn from(s: PublicKeyCredentialCreationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialCreationOptions {
    pub fn rp(&self) -> Any {
        self.inner.get("rp").as_::<Any>()
    }

    pub fn set_rp(&mut self, value: Any) {
        self.inner.set("rp", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn user(&self) -> Any {
        self.inner.get("user").as_::<Any>()
    }

    pub fn set_user(&mut self, value: Any) {
        self.inner.set("user", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn pub_key_cred_params(&self) -> Sequence<Any> {
        self.inner.get("pubKeyCredParams").as_::<Sequence<Any>>()
    }

    pub fn set_pub_key_cred_params(&mut self, value: Sequence<Any>) {
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

    pub fn set_exclude_credentials(&mut self, value: Sequence<Any>) {
        self.inner.set("excludeCredentials", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn authenticator_selection(&self) -> Any {
        self.inner.get("authenticatorSelection").as_::<Any>()
    }

    pub fn set_authenticator_selection(&mut self, value: Any) {
        self.inner.set("authenticatorSelection", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn hints(&self) -> Sequence<DOMString> {
        self.inner.get("hints").as_::<Sequence<DOMString>>()
    }

    pub fn set_hints(&mut self, value: Sequence<DOMString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn attestation(&self) -> DOMString {
        self.inner.get("attestation").as_::<DOMString>()
    }

    pub fn set_attestation(&mut self, value: DOMString) {
        self.inner.set("attestation", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn attestation_formats(&self) -> Sequence<DOMString> {
        self.inner
            .get("attestationFormats")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_attestation_formats(&mut self, value: Sequence<DOMString>) {
        self.inner.set("attestationFormats", value);
    }
}
impl PublicKeyCredentialCreationOptions {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: Any) {
        self.inner.set("extensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialCreationOptionsJSON {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialCreationOptionsJSON {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialCreationOptionsJSON { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PublicKeyCredentialCreationOptionsJSON {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PublicKeyCredentialCreationOptionsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PublicKeyCredentialCreationOptionsJSON {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PublicKeyCredentialCreationOptionsJSON {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialCreationOptionsJSON> for emlite::Val {
    fn from(s: PublicKeyCredentialCreationOptionsJSON) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialCreationOptionsJSON {
    pub fn rp(&self) -> Any {
        self.inner.get("rp").as_::<Any>()
    }

    pub fn set_rp(&mut self, value: Any) {
        self.inner.set("rp", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn user(&self) -> Any {
        self.inner.get("user").as_::<Any>()
    }

    pub fn set_user(&mut self, value: Any) {
        self.inner.set("user", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: Any) {
        self.inner.set("challenge", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn pub_key_cred_params(&self) -> Sequence<Any> {
        self.inner.get("pubKeyCredParams").as_::<Sequence<Any>>()
    }

    pub fn set_pub_key_cred_params(&mut self, value: Sequence<Any>) {
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

    pub fn set_exclude_credentials(&mut self, value: Sequence<Any>) {
        self.inner.set("excludeCredentials", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn authenticator_selection(&self) -> Any {
        self.inner.get("authenticatorSelection").as_::<Any>()
    }

    pub fn set_authenticator_selection(&mut self, value: Any) {
        self.inner.set("authenticatorSelection", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn hints(&self) -> Sequence<DOMString> {
        self.inner.get("hints").as_::<Sequence<DOMString>>()
    }

    pub fn set_hints(&mut self, value: Sequence<DOMString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn attestation(&self) -> DOMString {
        self.inner.get("attestation").as_::<DOMString>()
    }

    pub fn set_attestation(&mut self, value: DOMString) {
        self.inner.set("attestation", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn attestation_formats(&self) -> Sequence<DOMString> {
        self.inner
            .get("attestationFormats")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_attestation_formats(&mut self, value: Sequence<DOMString>) {
        self.inner.set("attestationFormats", value);
    }
}
impl PublicKeyCredentialCreationOptionsJSON {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: Any) {
        self.inner.set("extensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialRequestOptions {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PublicKeyCredentialRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PublicKeyCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PublicKeyCredentialRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PublicKeyCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialRequestOptions> for emlite::Val {
    fn from(s: PublicKeyCredentialRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialRequestOptions {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: Any) {
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
    pub fn rp_id(&self) -> DOMString {
        self.inner.get("rpId").as_::<DOMString>()
    }

    pub fn set_rp_id(&mut self, value: DOMString) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn allow_credentials(&self) -> Sequence<Any> {
        self.inner.get("allowCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_allow_credentials(&mut self, value: Sequence<Any>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn user_verification(&self) -> DOMString {
        self.inner.get("userVerification").as_::<DOMString>()
    }

    pub fn set_user_verification(&mut self, value: DOMString) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn hints(&self) -> Sequence<DOMString> {
        self.inner.get("hints").as_::<Sequence<DOMString>>()
    }

    pub fn set_hints(&mut self, value: Sequence<DOMString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptions {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: Any) {
        self.inner.set("extensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialRequestOptionsJSON {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialRequestOptionsJSON {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialRequestOptionsJSON { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PublicKeyCredentialRequestOptionsJSON {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PublicKeyCredentialRequestOptionsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PublicKeyCredentialRequestOptionsJSON {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PublicKeyCredentialRequestOptionsJSON {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialRequestOptionsJSON> for emlite::Val {
    fn from(s: PublicKeyCredentialRequestOptionsJSON) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialRequestOptionsJSON {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: Any) {
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
    pub fn rp_id(&self) -> DOMString {
        self.inner.get("rpId").as_::<DOMString>()
    }

    pub fn set_rp_id(&mut self, value: DOMString) {
        self.inner.set("rpId", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn allow_credentials(&self) -> Sequence<Any> {
        self.inner.get("allowCredentials").as_::<Sequence<Any>>()
    }

    pub fn set_allow_credentials(&mut self, value: Sequence<Any>) {
        self.inner.set("allowCredentials", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn user_verification(&self) -> DOMString {
        self.inner.get("userVerification").as_::<DOMString>()
    }

    pub fn set_user_verification(&mut self, value: DOMString) {
        self.inner.set("userVerification", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn hints(&self) -> Sequence<DOMString> {
        self.inner.get("hints").as_::<Sequence<DOMString>>()
    }

    pub fn set_hints(&mut self, value: Sequence<DOMString>) {
        self.inner.set("hints", value);
    }
}
impl PublicKeyCredentialRequestOptionsJSON {
    pub fn extensions(&self) -> Any {
        self.inner.get("extensions").as_::<Any>()
    }

    pub fn set_extensions(&mut self, value: Any) {
        self.inner.set("extensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnknownCredentialOptions {
    inner: emlite::Val,
}
impl FromVal for UnknownCredentialOptions {
    fn from_val(v: &emlite::Val) -> Self {
        UnknownCredentialOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for UnknownCredentialOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UnknownCredentialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for UnknownCredentialOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for UnknownCredentialOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<UnknownCredentialOptions> for emlite::Val {
    fn from(s: UnknownCredentialOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UnknownCredentialOptions {
    pub fn rp_id(&self) -> DOMString {
        self.inner.get("rpId").as_::<DOMString>()
    }

    pub fn set_rp_id(&mut self, value: DOMString) {
        self.inner.set("rpId", value);
    }
}
impl UnknownCredentialOptions {
    pub fn credential_id(&self) -> Any {
        self.inner.get("credentialId").as_::<Any>()
    }

    pub fn set_credential_id(&mut self, value: Any) {
        self.inner.set("credentialId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AllAcceptedCredentialsOptions {
    inner: emlite::Val,
}
impl FromVal for AllAcceptedCredentialsOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AllAcceptedCredentialsOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AllAcceptedCredentialsOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AllAcceptedCredentialsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AllAcceptedCredentialsOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AllAcceptedCredentialsOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AllAcceptedCredentialsOptions> for emlite::Val {
    fn from(s: AllAcceptedCredentialsOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AllAcceptedCredentialsOptions {
    pub fn rp_id(&self) -> DOMString {
        self.inner.get("rpId").as_::<DOMString>()
    }

    pub fn set_rp_id(&mut self, value: DOMString) {
        self.inner.set("rpId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    pub fn set_user_id(&mut self, value: Any) {
        self.inner.set("userId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    pub fn all_accepted_credential_ids(&self) -> Sequence<Any> {
        self.inner
            .get("allAcceptedCredentialIds")
            .as_::<Sequence<Any>>()
    }

    pub fn set_all_accepted_credential_ids(&mut self, value: Sequence<Any>) {
        self.inner.set("allAcceptedCredentialIds", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CurrentUserDetailsOptions {
    inner: emlite::Val,
}
impl FromVal for CurrentUserDetailsOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CurrentUserDetailsOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CurrentUserDetailsOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CurrentUserDetailsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CurrentUserDetailsOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CurrentUserDetailsOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CurrentUserDetailsOptions> for emlite::Val {
    fn from(s: CurrentUserDetailsOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CurrentUserDetailsOptions {
    pub fn rp_id(&self) -> DOMString {
        self.inner.get("rpId").as_::<DOMString>()
    }

    pub fn set_rp_id(&mut self, value: DOMString) {
        self.inner.set("rpId", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    pub fn set_user_id(&mut self, value: Any) {
        self.inner.set("userId", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl CurrentUserDetailsOptions {
    pub fn display_name(&self) -> DOMString {
        self.inner.get("displayName").as_::<DOMString>()
    }

    pub fn set_display_name(&mut self, value: DOMString) {
        self.inner.set("displayName", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredential {
    inner: Credential,
}
impl FromVal for PublicKeyCredential {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for PublicKeyCredential {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PublicKeyCredential {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PublicKeyCredential> for emlite::Val {
    fn from(s: PublicKeyCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PublicKeyCredential);

impl PublicKeyCredential {
    pub fn raw_id(&self) -> ArrayBuffer {
        self.inner.get("rawId").as_::<ArrayBuffer>()
    }
}
impl PublicKeyCredential {
    pub fn response(&self) -> AuthenticatorResponse {
        self.inner.get("response").as_::<AuthenticatorResponse>()
    }
}
impl PublicKeyCredential {
    pub fn authenticator_attachment(&self) -> DOMString {
        self.inner.get("authenticatorAttachment").as_::<DOMString>()
    }
}
impl PublicKeyCredential {
    pub fn get_client_extension_results(&self) -> AuthenticationExtensionsClientOutputs {
        self.inner
            .call("getClientExtensionResults", &[])
            .as_::<AuthenticationExtensionsClientOutputs>()
    }
}
impl PublicKeyCredential {
    pub fn is_conditional_mediation_available() -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("isConditionalMediationAvailable", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    pub fn to_json(&self) -> Any {
        self.inner.call("toJSON", &[]).as_::<Any>()
    }
}
impl PublicKeyCredential {
    pub fn is_user_verifying_platform_authenticator_available() -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("isUserVerifyingPlatformAuthenticatorAvailable", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    pub fn get_client_capabilities() -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("getClientCapabilities", &[])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    pub fn parse_creation_options_from_json(
        options: PublicKeyCredentialCreationOptionsJSON,
    ) -> PublicKeyCredentialCreationOptions {
        emlite::Val::global("PublicKeyCredential")
            .call("parseCreationOptionsFromJSON", &[options.into()])
            .as_::<PublicKeyCredentialCreationOptions>()
    }
}
impl PublicKeyCredential {
    pub fn parse_request_options_from_json(
        options: PublicKeyCredentialRequestOptionsJSON,
    ) -> PublicKeyCredentialRequestOptions {
        emlite::Val::global("PublicKeyCredential")
            .call("parseRequestOptionsFromJSON", &[options.into()])
            .as_::<PublicKeyCredentialRequestOptions>()
    }
}
impl PublicKeyCredential {
    pub fn signal_unknown_credential(options: UnknownCredentialOptions) -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("signalUnknownCredential", &[options.into()])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    pub fn signal_all_accepted_credentials(options: AllAcceptedCredentialsOptions) -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("signalAllAcceptedCredentials", &[options.into()])
            .as_::<Promise>()
    }
}
impl PublicKeyCredential {
    pub fn signal_current_user_details(options: CurrentUserDetailsOptions) -> Promise {
        emlite::Val::global("PublicKeyCredential")
            .call("signalCurrentUserDetails", &[options.into()])
            .as_::<Promise>()
    }
}
