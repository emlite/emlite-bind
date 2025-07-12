use super::*;

#[derive(Clone, Debug)]
pub struct BluetoothPermissionResult {
    inner: PermissionStatus,
}
impl FromVal for BluetoothPermissionResult {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothPermissionResult {
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
impl std::ops::Deref for BluetoothPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothPermissionResult> for emlite::Val {
    fn from(s: BluetoothPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothPermissionResult {
    pub fn devices(&self) -> jsbind::FrozenArray<BluetoothDevice> {
        self.inner
            .get("devices")
            .as_::<jsbind::FrozenArray<BluetoothDevice>>()
    }

    pub fn set_devices(&mut self, value: jsbind::FrozenArray<BluetoothDevice>) {
        self.inner.set("devices", value);
    }
}
