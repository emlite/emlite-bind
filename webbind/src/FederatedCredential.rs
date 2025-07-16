use super::*;

/// The FederatedCredential class.
/// [`FederatedCredential`](https://developer.mozilla.org/en-US/docs/Web/API/FederatedCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FederatedCredential {
    inner: Credential,
}
impl FromVal for FederatedCredential {
    fn from_val(v: &Any) -> Self {
        FederatedCredential {
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
impl core::ops::Deref for FederatedCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FederatedCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FederatedCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FederatedCredential {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FederatedCredential> for Any {
    fn from(s: FederatedCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FederatedCredential> for Any {
    fn from(s: &FederatedCredential) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FederatedCredential);

impl FederatedCredential {
    /// The `new FederatedCredential(..)` constructor, creating a new FederatedCredential instance
    pub fn new(data: &Any) -> FederatedCredential {
        Self {
            inner: Any::global("FederatedCredential")
                .new(&[data.into()])
                .as_::<Credential>(),
        }
    }
}
impl FederatedCredential {
    /// Getter of the `provider` attribute.
    /// [`FederatedCredential.provider`](https://developer.mozilla.org/en-US/docs/Web/API/FederatedCredential/provider)
    pub fn provider(&self) -> String {
        self.inner.get("provider").as_::<String>()
    }
}
impl FederatedCredential {
    /// Getter of the `protocol` attribute.
    /// [`FederatedCredential.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/FederatedCredential/protocol)
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }
}
impl FederatedCredential {
    /// Getter of the `name` attribute.
    /// [`FederatedCredential.name`](https://developer.mozilla.org/en-US/docs/Web/API/FederatedCredential/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl FederatedCredential {
    /// Getter of the `iconURL` attribute.
    /// [`FederatedCredential.iconURL`](https://developer.mozilla.org/en-US/docs/Web/API/FederatedCredential/iconURL)
    pub fn icon_url(&self) -> String {
        self.inner.get("iconURL").as_::<String>()
    }
}
