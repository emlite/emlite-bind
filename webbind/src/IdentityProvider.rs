use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityResolveOptions {
    inner: Any,
}
impl FromVal for IdentityResolveOptions {
    fn from_val(v: &Any) -> Self {
        IdentityResolveOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityResolveOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityResolveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityResolveOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityResolveOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityResolveOptions> for Any {
    fn from(s: IdentityResolveOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityResolveOptions> for Any {
    fn from(s: &IdentityResolveOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityResolveOptions {
    pub fn account_id(&self) -> String {
        self.inner.get("accountId").as_::<String>()
    }

    pub fn set_account_id(&mut self, value: &str) {
        self.inner.set("accountId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityUserInfo {
    inner: Any,
}
impl FromVal for IdentityUserInfo {
    fn from_val(v: &Any) -> Self {
        IdentityUserInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityUserInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityUserInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityUserInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityUserInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityUserInfo> for Any {
    fn from(s: IdentityUserInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityUserInfo> for Any {
    fn from(s: &IdentityUserInfo) -> Any {
        s.inner.clone()
    }
}

impl IdentityUserInfo {
    pub fn email(&self) -> String {
        self.inner.get("email").as_::<String>()
    }

    pub fn set_email(&mut self, value: &str) {
        self.inner.set("email", value);
    }
}
impl IdentityUserInfo {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl IdentityUserInfo {
    pub fn given_name(&self) -> String {
        self.inner.get("givenName").as_::<String>()
    }

    pub fn set_given_name(&mut self, value: &str) {
        self.inner.set("givenName", value);
    }
}
impl IdentityUserInfo {
    pub fn picture(&self) -> String {
        self.inner.get("picture").as_::<String>()
    }

    pub fn set_picture(&mut self, value: &str) {
        self.inner.set("picture", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderConfig {
    inner: Any,
}
impl FromVal for IdentityProviderConfig {
    fn from_val(v: &Any) -> Self {
        IdentityProviderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderConfig> for Any {
    fn from(s: IdentityProviderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderConfig> for Any {
    fn from(s: &IdentityProviderConfig) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderConfig {
    pub fn config_url(&self) -> String {
        self.inner.get("configURL").as_::<String>()
    }

    pub fn set_config_url(&mut self, value: &str) {
        self.inner.set("configURL", value);
    }
}
impl IdentityProviderConfig {
    pub fn client_id(&self) -> String {
        self.inner.get("clientId").as_::<String>()
    }

    pub fn set_client_id(&mut self, value: &str) {
        self.inner.set("clientId", value);
    }
}
/// The IdentityProvider class.
/// [`IdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProvider {
    inner: Any,
}
impl FromVal for IdentityProvider {
    fn from_val(v: &Any) -> Self {
        IdentityProvider {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProvider {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProvider {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProvider {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProvider> for Any {
    fn from(s: IdentityProvider) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProvider> for Any {
    fn from(s: &IdentityProvider) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IdentityProvider);

impl IdentityProvider {
    /// The close method.
    /// [`IdentityProvider.close`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/close)
    pub fn close() -> Undefined {
        Any::global("IdentityProvider")
            .call("close", &[])
            .as_::<Undefined>()
    }
}
impl IdentityProvider {
    /// The resolve method.
    /// [`IdentityProvider.resolve`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/resolve)
    pub fn resolve0(token: &str) -> Promise {
        Any::global("IdentityProvider")
            .call("resolve", &[token.into()])
            .as_::<Promise>()
    }
    /// The resolve method.
    /// [`IdentityProvider.resolve`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/resolve)
    pub fn resolve1(token: &str, options: &IdentityResolveOptions) -> Promise {
        Any::global("IdentityProvider")
            .call("resolve", &[token.into(), options.into()])
            .as_::<Promise>()
    }
}
impl IdentityProvider {
    /// The getUserInfo method.
    /// [`IdentityProvider.getUserInfo`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/getUserInfo)
    pub fn get_user_info(config: &IdentityProviderConfig) -> Promise {
        Any::global("IdentityProvider")
            .call("getUserInfo", &[config.into()])
            .as_::<Promise>()
    }
}
