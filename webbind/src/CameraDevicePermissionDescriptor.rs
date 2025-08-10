use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CameraDevicePermissionDescriptor {
    inner: Any,
}
impl FromVal for CameraDevicePermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        CameraDevicePermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CameraDevicePermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CameraDevicePermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CameraDevicePermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CameraDevicePermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CameraDevicePermissionDescriptor> for Any {
    fn from(s: CameraDevicePermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CameraDevicePermissionDescriptor> for Any {
    fn from(s: &CameraDevicePermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl CameraDevicePermissionDescriptor {
    pub fn pan_tilt_zoom(&self) -> bool {
        self.inner.get("panTiltZoom").as_::<bool>()
    }

    pub fn set_pan_tilt_zoom(&mut self, value: bool) {
        self.inner.set("panTiltZoom", value);
    }
}
