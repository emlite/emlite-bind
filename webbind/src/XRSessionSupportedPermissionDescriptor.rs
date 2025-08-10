use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionSupportedPermissionDescriptor {
    inner: Any,
}
impl FromVal for XRSessionSupportedPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        XRSessionSupportedPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSessionSupportedPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSessionSupportedPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRSessionSupportedPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSessionSupportedPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSessionSupportedPermissionDescriptor> for Any {
    fn from(s: XRSessionSupportedPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSessionSupportedPermissionDescriptor> for Any {
    fn from(s: &XRSessionSupportedPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl XRSessionSupportedPermissionDescriptor {
    pub fn mode(&self) -> XRSessionMode {
        self.inner.get("mode").as_::<XRSessionMode>()
    }

    pub fn set_mode(&mut self, value: &XRSessionMode) {
        self.inner.set("mode", value);
    }
}
