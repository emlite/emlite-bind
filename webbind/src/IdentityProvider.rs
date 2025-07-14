use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IdentityResolveOptions {
    inner: emlite::Val,
}
impl FromVal for IdentityResolveOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityResolveOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityResolveOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityResolveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityResolveOptions> for emlite::Val {
    fn from(s: IdentityResolveOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityResolveOptions {
    pub fn account_id(&self) -> jsbind::USVString {
        self.inner.get("accountId").as_::<jsbind::USVString>()
    }

    pub fn set_account_id(&mut self, value: jsbind::USVString) {
        self.inner.set("accountId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IdentityUserInfo {
    inner: emlite::Val,
}
impl FromVal for IdentityUserInfo {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityUserInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityUserInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityUserInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityUserInfo> for emlite::Val {
    fn from(s: IdentityUserInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityUserInfo {
    pub fn email(&self) -> jsbind::USVString {
        self.inner.get("email").as_::<jsbind::USVString>()
    }

    pub fn set_email(&mut self, value: jsbind::USVString) {
        self.inner.set("email", value);
    }
}
impl IdentityUserInfo {
    pub fn name(&self) -> jsbind::USVString {
        self.inner.get("name").as_::<jsbind::USVString>()
    }

    pub fn set_name(&mut self, value: jsbind::USVString) {
        self.inner.set("name", value);
    }
}
impl IdentityUserInfo {
    pub fn given_name(&self) -> jsbind::USVString {
        self.inner.get("givenName").as_::<jsbind::USVString>()
    }

    pub fn set_given_name(&mut self, value: jsbind::USVString) {
        self.inner.set("givenName", value);
    }
}
impl IdentityUserInfo {
    pub fn picture(&self) -> jsbind::USVString {
        self.inner.get("picture").as_::<jsbind::USVString>()
    }

    pub fn set_picture(&mut self, value: jsbind::USVString) {
        self.inner.set("picture", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IdentityProviderConfig {
    inner: emlite::Val,
}
impl FromVal for IdentityProviderConfig {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityProviderConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityProviderConfig> for emlite::Val {
    fn from(s: IdentityProviderConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityProviderConfig {
    pub fn config_url(&self) -> jsbind::USVString {
        self.inner.get("configURL").as_::<jsbind::USVString>()
    }

    pub fn set_config_url(&mut self, value: jsbind::USVString) {
        self.inner.set("configURL", value);
    }
}
impl IdentityProviderConfig {
    pub fn client_id(&self) -> jsbind::USVString {
        self.inner.get("clientId").as_::<jsbind::USVString>()
    }

    pub fn set_client_id(&mut self, value: jsbind::USVString) {
        self.inner.set("clientId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IdentityProvider {
    inner: emlite::Val,
}
impl FromVal for IdentityProvider {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityProvider {
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
impl core::ops::Deref for IdentityProvider {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityProvider> for emlite::Val {
    fn from(s: IdentityProvider) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityProvider {
    pub fn close() -> jsbind::Undefined {
        emlite::Val::global("identityprovider")
            .call("close", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl IdentityProvider {
    pub fn resolve0(token: jsbind::DOMString) -> jsbind::Promise {
        emlite::Val::global("identityprovider")
            .call("resolve", &[token.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn resolve1(token: jsbind::DOMString, options: IdentityResolveOptions) -> jsbind::Promise {
        emlite::Val::global("identityprovider")
            .call("resolve", &[token.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl IdentityProvider {
    pub fn get_user_info(config: IdentityProviderConfig) -> jsbind::Promise {
        emlite::Val::global("identityprovider")
            .call("getUserInfo", &[config.into()])
            .as_::<jsbind::Promise>()
    }
}
