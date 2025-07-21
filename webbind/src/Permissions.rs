use super::*;

/// The Permissions class.
/// [`Permissions`](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Permissions {
    inner: Any,
}
impl FromVal for Permissions {
    fn from_val(v: &Any) -> Self {
        Permissions {
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
impl core::ops::Deref for Permissions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Permissions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Permissions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Permissions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Permissions> for Any {
    fn from(s: Permissions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Permissions> for Any {
    fn from(s: &Permissions) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Permissions);

impl Permissions {
    /// The query method.
    /// [`Permissions.query`](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)
    pub fn query(&self, permission_desc: &Object) -> Promise<PermissionStatus> {
        self.inner
            .call("query", &[permission_desc.into()])
            .as_::<Promise<PermissionStatus>>()
    }
}
impl Permissions {
    /// The request method.
    /// [`Permissions.request`](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/request)
    pub fn request(&self, permission_desc: &Object) -> Promise<PermissionStatus> {
        self.inner
            .call("request", &[permission_desc.into()])
            .as_::<Promise<PermissionStatus>>()
    }
}
impl Permissions {
    /// The revoke method.
    /// [`Permissions.revoke`](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)
    pub fn revoke(&self, permission_desc: &Object) -> Promise<PermissionStatus> {
        self.inner
            .call("revoke", &[permission_desc.into()])
            .as_::<Promise<PermissionStatus>>()
    }
}
