use super::*;

/// The BluetoothServiceDataMap class.
/// [`BluetoothServiceDataMap`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothServiceDataMap {
    inner: Any,
}

impl FromVal for BluetoothServiceDataMap {
    fn from_val(v: &Any) -> Self {
        BluetoothServiceDataMap {
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

impl core::ops::Deref for BluetoothServiceDataMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothServiceDataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothServiceDataMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothServiceDataMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothServiceDataMap> for Any {
    fn from(s: BluetoothServiceDataMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothServiceDataMap> for Any {
    fn from(s: &BluetoothServiceDataMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothServiceDataMap);
