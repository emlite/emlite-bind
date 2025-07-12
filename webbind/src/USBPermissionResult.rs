use super::*;

#[derive(Clone, Debug)]
pub struct USBPermissionResult {
    inner: PermissionStatus,
}
impl FromVal for USBPermissionResult {
    fn from_val(v: &emlite::Val) -> Self {
        USBPermissionResult {
            inner: PermissionStatus::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBPermissionResult> for emlite::Val {
    fn from(s: USBPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBPermissionResult {
    pub fn devices(&self) -> jsbind::FrozenArray<USBDevice> {
        self.inner
            .get("devices")
            .as_::<jsbind::FrozenArray<USBDevice>>()
    }

    pub fn set_devices(&mut self, value: jsbind::FrozenArray<USBDevice>) {
        self.inner.set("devices", value);
    }
}
