use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PrivateNetworkAccessPermissionDescriptor {
    inner: Any,
}
impl FromVal for PrivateNetworkAccessPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        PrivateNetworkAccessPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PrivateNetworkAccessPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PrivateNetworkAccessPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PrivateNetworkAccessPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PrivateNetworkAccessPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PrivateNetworkAccessPermissionDescriptor> for Any {
    fn from(s: PrivateNetworkAccessPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PrivateNetworkAccessPermissionDescriptor> for Any {
    fn from(s: &PrivateNetworkAccessPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl PrivateNetworkAccessPermissionDescriptor {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
