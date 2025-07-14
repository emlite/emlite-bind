use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<IdentityCredentialDisconnectOptions> for emlite::Val {
    fn from(s: IdentityCredentialDisconnectOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityCredentialDisconnectOptions {
    pub fn account_hint(&self) -> jsbind::USVString {
        self.inner.get("accountHint").as_::<jsbind::USVString>()
    }

    pub fn set_account_hint(&mut self, value: jsbind::USVString) {
        self.inner.set("accountHint", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<IdentityCredential> for emlite::Val {
    fn from(s: IdentityCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityCredential {
    pub fn disconnect(options: IdentityCredentialDisconnectOptions) -> jsbind::Promise {
        emlite::Val::global("identitycredential")
            .call("disconnect", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl IdentityCredential {
    pub fn token(&self) -> jsbind::USVString {
        self.inner.get("token").as_::<jsbind::USVString>()
    }
}
impl IdentityCredential {
    pub fn is_auto_selected(&self) -> bool {
        self.inner.get("isAutoSelected").as_::<bool>()
    }
}
impl IdentityCredential {
    pub fn config_url(&self) -> jsbind::USVString {
        self.inner.get("configURL").as_::<jsbind::USVString>()
    }
}
