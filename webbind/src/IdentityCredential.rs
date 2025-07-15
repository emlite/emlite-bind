use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialDisconnectOptions {
    inner: emlite::Val,
}
impl FromVal for IdentityCredentialDisconnectOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityCredentialDisconnectOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityCredentialDisconnectOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityCredentialDisconnectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IdentityCredentialDisconnectOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityCredentialDisconnectOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IdentityCredentialDisconnectOptions> for emlite::Val {
    fn from(s: IdentityCredentialDisconnectOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityCredentialDisconnectOptions {
    pub fn account_hint(&self) -> USVString {
        self.inner.get("accountHint").as_::<USVString>()
    }

    pub fn set_account_hint(&mut self, value: USVString) {
        self.inner.set("accountHint", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredential {
    inner: Credential,
}
impl FromVal for IdentityCredential {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityCredential {
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
impl core::ops::Deref for IdentityCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IdentityCredential {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityCredential {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IdentityCredential> for emlite::Val {
    fn from(s: IdentityCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IdentityCredential);

impl IdentityCredential {
    pub fn disconnect(options: IdentityCredentialDisconnectOptions) -> Promise {
        emlite::Val::global("IdentityCredential")
            .call("disconnect", &[options.into()])
            .as_::<Promise>()
    }
}
impl IdentityCredential {
    pub fn token(&self) -> USVString {
        self.inner.get("token").as_::<USVString>()
    }
}
impl IdentityCredential {
    pub fn is_auto_selected(&self) -> bool {
        self.inner.get("isAutoSelected").as_::<bool>()
    }
}
impl IdentityCredential {
    pub fn config_url(&self) -> USVString {
        self.inner.get("configURL").as_::<USVString>()
    }
}
