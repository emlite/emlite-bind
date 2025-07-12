use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for BluetoothLEScanPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothLEScanPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothLEScanPermissionResult> for emlite::Val {
    fn from(s: BluetoothLEScanPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothLEScanPermissionResult {
    pub fn scans(&self) -> jsbind::FrozenArray<BluetoothLEScan> {
        self.inner
            .get("scans")
            .as_::<jsbind::FrozenArray<BluetoothLEScan>>()
    }

    pub fn set_scans(&mut self, value: jsbind::FrozenArray<BluetoothLEScan>) {
        self.inner.set("scans", value);
    }
}
