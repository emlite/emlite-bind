use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for USBPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBPermissionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBPermissionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBPermissionResult> for emlite::Val {
    fn from(s: USBPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&USBPermissionResult> for emlite::Val {
    fn from(s: &USBPermissionResult) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBPermissionResult);

impl USBPermissionResult {
    pub fn devices(&self) -> FrozenArray<USBDevice> {
        self.inner.get("devices").as_::<FrozenArray<USBDevice>>()
    }

    pub fn set_devices(&mut self, value: &FrozenArray<USBDevice>) {
        self.inner.set("devices", value);
    }
}
