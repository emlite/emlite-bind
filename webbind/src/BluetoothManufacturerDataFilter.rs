use super::*;




/// The BluetoothManufacturerDataFilter class.
/// [`BluetoothManufacturerDataFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataFilter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothManufacturerDataFilter {
    inner: Any,
}

impl FromVal for BluetoothManufacturerDataFilter {
    fn from_val(v: &Any) -> Self {
        BluetoothManufacturerDataFilter { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothManufacturerDataFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothManufacturerDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothManufacturerDataFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothManufacturerDataFilter {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothManufacturerDataFilter> for Any {
    fn from(s: BluetoothManufacturerDataFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothManufacturerDataFilter> for Any {
    fn from(s: &BluetoothManufacturerDataFilter) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothManufacturerDataFilter);



impl BluetoothManufacturerDataFilter {
    /// The `new BluetoothManufacturerDataFilter(..)` constructor, creating a new BluetoothManufacturerDataFilter instance
    pub fn new0() -> BluetoothManufacturerDataFilter {
        Self {
            inner: Any::global("BluetoothManufacturerDataFilter").new(&[]).as_::<Any>(),
        }
    }

    /// The `new BluetoothManufacturerDataFilter(..)` constructor, creating a new BluetoothManufacturerDataFilter instance
    pub fn new1(init: &Object) -> BluetoothManufacturerDataFilter {
        Self {
            inner: Any::global("BluetoothManufacturerDataFilter").new(&[init.into()]).as_::<Any>(),
        }
    }

}
