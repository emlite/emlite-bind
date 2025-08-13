use super::*;




/// The BluetoothUUID class.
/// [`BluetoothUUID`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothUUID {
    inner: Any,
}

impl FromVal for BluetoothUUID {
    fn from_val(v: &Any) -> Self {
        BluetoothUUID { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothUUID {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothUUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothUUID {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothUUID {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothUUID> for Any {
    fn from(s: BluetoothUUID) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothUUID> for Any {
    fn from(s: &BluetoothUUID) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothUUID);


impl BluetoothUUID {
    /// The getService method.
    /// [`BluetoothUUID.getService`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getService)
    pub fn get_service(name: &Any) -> Any {
        Any::global("BluetoothUUID").call("getService", &[name.into(), ]).as_::<Any>()
    }
}
impl BluetoothUUID {
    /// The getCharacteristic method.
    /// [`BluetoothUUID.getCharacteristic`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getCharacteristic)
    pub fn get_characteristic(name: &Any) -> Any {
        Any::global("BluetoothUUID").call("getCharacteristic", &[name.into(), ]).as_::<Any>()
    }
}
impl BluetoothUUID {
    /// The getDescriptor method.
    /// [`BluetoothUUID.getDescriptor`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getDescriptor)
    pub fn get_descriptor(name: &Any) -> Any {
        Any::global("BluetoothUUID").call("getDescriptor", &[name.into(), ]).as_::<Any>()
    }
}
impl BluetoothUUID {
    /// The canonicalUUID method.
    /// [`BluetoothUUID.canonicalUUID`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/canonicalUUID)
    pub fn canonical_uuid(alias: u32) -> Any {
        Any::global("BluetoothUUID").call("canonicalUUID", &[alias.into(), ]).as_::<Any>()
    }
}
