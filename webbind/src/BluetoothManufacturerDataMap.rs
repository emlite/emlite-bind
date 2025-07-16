use super::*;

/// The BluetoothManufacturerDataMap class.
/// [`BluetoothManufacturerDataMap`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothManufacturerDataMap {
    inner: Any,
}
impl FromVal for BluetoothManufacturerDataMap {
    fn from_val(v: &Any) -> Self {
        BluetoothManufacturerDataMap {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothManufacturerDataMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothManufacturerDataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothManufacturerDataMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothManufacturerDataMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothManufacturerDataMap> for Any {
    fn from(s: BluetoothManufacturerDataMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothManufacturerDataMap> for Any {
    fn from(s: &BluetoothManufacturerDataMap) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothManufacturerDataMap);
