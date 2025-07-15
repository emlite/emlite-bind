use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialRequestOptions {
    inner: emlite::Val,
}
impl FromVal for CredentialRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CredentialRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CredentialRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CredentialRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CredentialRequestOptions> for emlite::Val {
    fn from(s: CredentialRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CredentialRequestOptions> for emlite::Val {
    fn from(s: &CredentialRequestOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl CredentialRequestOptions {
    pub fn public_key(&self) -> PublicKeyCredentialRequestOptions {
        self.inner
            .get("publicKey")
            .as_::<PublicKeyCredentialRequestOptions>()
    }

    pub fn set_public_key(&mut self, value: &PublicKeyCredentialRequestOptions) {
        self.inner.set("publicKey", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialCreationOptions {
    inner: emlite::Val,
}
impl FromVal for CredentialCreationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CredentialCreationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CredentialCreationOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CredentialCreationOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CredentialCreationOptions> for emlite::Val {
    fn from(s: CredentialCreationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CredentialCreationOptions> for emlite::Val {
    fn from(s: &CredentialCreationOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl CredentialCreationOptions {
    pub fn public_key(&self) -> PublicKeyCredentialCreationOptions {
        self.inner
            .get("publicKey")
            .as_::<PublicKeyCredentialCreationOptions>()
    }

    pub fn set_public_key(&mut self, value: &PublicKeyCredentialCreationOptions) {
        self.inner.set("publicKey", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialsContainer {
    inner: emlite::Val,
}
impl FromVal for CredentialsContainer {
    fn from_val(v: &emlite::Val) -> Self {
        CredentialsContainer {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CredentialsContainer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialsContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CredentialsContainer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CredentialsContainer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CredentialsContainer> for emlite::Val {
    fn from(s: CredentialsContainer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CredentialsContainer> for emlite::Val {
    fn from(s: &CredentialsContainer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CredentialsContainer);

impl CredentialsContainer {
    pub fn get0(&self) -> Promise {
        self.inner.call("get", &[]).as_::<Promise>()
    }

    pub fn get1(&self, options: &CredentialRequestOptions) -> Promise {
        self.inner.call("get", &[options.into()]).as_::<Promise>()
    }
}
impl CredentialsContainer {
    pub fn store(&self, credential: &Credential) -> Promise {
        self.inner
            .call("store", &[credential.into()])
            .as_::<Promise>()
    }
}
impl CredentialsContainer {
    pub fn create0(&self) -> Promise {
        self.inner.call("create", &[]).as_::<Promise>()
    }

    pub fn create1(&self, options: &CredentialCreationOptions) -> Promise {
        self.inner
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl CredentialsContainer {
    pub fn prevent_silent_access(&self) -> Promise {
        self.inner.call("preventSilentAccess", &[]).as_::<Promise>()
    }
}
