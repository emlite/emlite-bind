use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanPermissionResult {
    inner: PermissionStatus,
}
impl FromVal for BluetoothLEScanPermissionResult {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothLEScanPermissionResult {
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
impl core::ops::Deref for BluetoothLEScanPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothLEScanPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothLEScanPermissionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothLEScanPermissionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothLEScanPermissionResult> for emlite::Val {
    fn from(s: BluetoothLEScanPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothLEScanPermissionResult> for emlite::Val {
    fn from(s: &BluetoothLEScanPermissionResult) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothLEScanPermissionResult);

impl BluetoothLEScanPermissionResult {
    pub fn scans(&self) -> FrozenArray<BluetoothLEScan> {
        self.inner
            .get("scans")
            .as_::<FrozenArray<BluetoothLEScan>>()
    }

    pub fn set_scans(&mut self, value: &FrozenArray<BluetoothLEScan>) {
        self.inner.set("scans", value);
    }
}
