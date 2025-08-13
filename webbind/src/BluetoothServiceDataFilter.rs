use super::*;




/// The BluetoothServiceDataFilter class.
/// [`BluetoothServiceDataFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataFilter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothServiceDataFilter {
    inner: Any,
}

impl FromVal for BluetoothServiceDataFilter {
    fn from_val(v: &Any) -> Self {
        BluetoothServiceDataFilter { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothServiceDataFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothServiceDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothServiceDataFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothServiceDataFilter {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothServiceDataFilter> for Any {
    fn from(s: BluetoothServiceDataFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothServiceDataFilter> for Any {
    fn from(s: &BluetoothServiceDataFilter) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothServiceDataFilter);



impl BluetoothServiceDataFilter {
    /// The `new BluetoothServiceDataFilter(..)` constructor, creating a new BluetoothServiceDataFilter instance
    pub fn new0() -> BluetoothServiceDataFilter {
        Self {
            inner: Any::global("BluetoothServiceDataFilter").new(&[]).as_::<Any>(),
        }
    }

    /// The `new BluetoothServiceDataFilter(..)` constructor, creating a new BluetoothServiceDataFilter instance
    pub fn new1(init: &Object) -> BluetoothServiceDataFilter {
        Self {
            inner: Any::global("BluetoothServiceDataFilter").new(&[init.into()]).as_::<Any>(),
        }
    }

}
