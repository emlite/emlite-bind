use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothPermissionResult {
    inner: PermissionStatus,
}
impl FromVal for BluetoothPermissionResult {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothPermissionResult { inner: PermissionStatus::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothPermissionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothPermissionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<BluetoothPermissionResult> for emlite::Val {
    fn from(s: BluetoothPermissionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothPermissionResult);


impl BluetoothPermissionResult {
    pub fn devices(&self) -> FrozenArray<BluetoothDevice> {
        self.inner.get("devices").as_::<FrozenArray<BluetoothDevice>>()
    }

    pub fn set_devices(&mut self, value: FrozenArray<BluetoothDevice>) {
        self.inner.set("devices", value);
    }

}
