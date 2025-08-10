use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionDescriptor {
    inner: Any,
}
impl FromVal for PermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        PermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PermissionDescriptor> for Any {
    fn from(s: PermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PermissionDescriptor> for Any {
    fn from(s: &PermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl PermissionDescriptor {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
