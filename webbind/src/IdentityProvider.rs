use super::*;

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
    pub fn resolve(token: &Any) -> Promise<Undefined> {
        Any::global("IdentityProvider")
            .call("resolve", &[token.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl IdentityProvider {
    /// The resolve method.
    /// [`IdentityProvider.resolve`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/resolve)
    pub fn resolve_with_options(
        token: &Any,
        options: &IdentityResolveOptions,
    ) -> Promise<Undefined> {
        Any::global("IdentityProvider")
            .call("resolve", &[token.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl IdentityProvider {
    /// The getUserInfo method.
    /// [`IdentityProvider.getUserInfo`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityProvider/getUserInfo)
    pub fn get_user_info(config: &IdentityProviderConfig) -> Promise<TypedArray<IdentityUserInfo>> {
        Any::global("IdentityProvider")
            .call("getUserInfo", &[config.into()])
            .as_::<Promise<TypedArray<IdentityUserInfo>>>()
    }
}
