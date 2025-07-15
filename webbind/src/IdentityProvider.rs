use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for IdentityResolveOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityResolveOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&IdentityResolveOptions> for emlite::Val {
    fn from(s: &IdentityResolveOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl IdentityResolveOptions {
    pub fn account_id(&self) -> USVString {
        self.inner.get("accountId").as_::<USVString>()
    }

    pub fn set_account_id(&mut self, value: USVString) {
        self.inner.set("accountId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for IdentityUserInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityUserInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&IdentityUserInfo> for emlite::Val {
    fn from(s: &IdentityUserInfo) -> emlite::Val {
        s.inner.clone()
    }
}

impl IdentityUserInfo {
    pub fn email(&self) -> USVString {
        self.inner.get("email").as_::<USVString>()
    }

    pub fn set_email(&mut self, value: USVString) {
        self.inner.set("email", value);
    }
}
impl IdentityUserInfo {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }
}
impl IdentityUserInfo {
    pub fn given_name(&self) -> USVString {
        self.inner.get("givenName").as_::<USVString>()
    }

    pub fn set_given_name(&mut self, value: USVString) {
        self.inner.set("givenName", value);
    }
}
impl IdentityUserInfo {
    pub fn picture(&self) -> USVString {
        self.inner.get("picture").as_::<USVString>()
    }

    pub fn set_picture(&mut self, value: USVString) {
        self.inner.set("picture", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for IdentityProviderConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityProviderConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&IdentityProviderConfig> for emlite::Val {
    fn from(s: &IdentityProviderConfig) -> emlite::Val {
        s.inner.clone()
    }
}

impl IdentityProviderConfig {
    pub fn config_url(&self) -> USVString {
        self.inner.get("configURL").as_::<USVString>()
    }

    pub fn set_config_url(&mut self, value: USVString) {
        self.inner.set("configURL", value);
    }
}
impl IdentityProviderConfig {
    pub fn client_id(&self) -> USVString {
        self.inner.get("clientId").as_::<USVString>()
    }

    pub fn set_client_id(&mut self, value: USVString) {
        self.inner.set("clientId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for IdentityProvider {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityProvider {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&IdentityProvider> for emlite::Val {
    fn from(s: &IdentityProvider) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IdentityProvider);

impl IdentityProvider {
    pub fn close() -> Undefined {
        emlite::Val::global("IdentityProvider")
            .call("close", &[])
            .as_::<Undefined>()
    }
}
impl IdentityProvider {
    pub fn resolve0(token: DOMString) -> Promise {
        emlite::Val::global("IdentityProvider")
            .call("resolve", &[token.into()])
            .as_::<Promise>()
    }

    pub fn resolve1(token: DOMString, options: IdentityResolveOptions) -> Promise {
        emlite::Val::global("IdentityProvider")
            .call("resolve", &[token.into(), options.into()])
            .as_::<Promise>()
    }
}
impl IdentityProvider {
    pub fn get_user_info(config: IdentityProviderConfig) -> Promise {
        emlite::Val::global("IdentityProvider")
            .call("getUserInfo", &[config.into()])
            .as_::<Promise>()
    }
}
