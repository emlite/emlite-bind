use super::*;

/// The TopLevelStorageAccessPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TopLevelStorageAccessPermissionDescriptor {
    inner: Any,
}

impl FromVal for TopLevelStorageAccessPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        TopLevelStorageAccessPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TopLevelStorageAccessPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TopLevelStorageAccessPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TopLevelStorageAccessPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TopLevelStorageAccessPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TopLevelStorageAccessPermissionDescriptor> for Any {
    fn from(s: TopLevelStorageAccessPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TopLevelStorageAccessPermissionDescriptor> for Any {
    fn from(s: &TopLevelStorageAccessPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl TopLevelStorageAccessPermissionDescriptor {
    /// Getter of the `requestedOrigin` attribute.
    pub fn requested_origin(&self) -> JsString {
        self.inner.get("requestedOrigin").as_::<JsString>()
    }

    /// Setter of the `requestedOrigin` attribute.
    pub fn set_requested_origin(&mut self, value: &JsString) {
        self.inner.set("requestedOrigin", value);
    }
}
