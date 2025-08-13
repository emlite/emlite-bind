use super::*;




/// The BluetoothRemoteGATTDescriptor class.
/// [`BluetoothRemoteGATTDescriptor`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTDescriptor {
    inner: Any,
}

impl FromVal for BluetoothRemoteGATTDescriptor {
    fn from_val(v: &Any) -> Self {
        BluetoothRemoteGATTDescriptor { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothRemoteGATTDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothRemoteGATTDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothRemoteGATTDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothRemoteGATTDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothRemoteGATTDescriptor> for Any {
    fn from(s: BluetoothRemoteGATTDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothRemoteGATTDescriptor> for Any {
    fn from(s: &BluetoothRemoteGATTDescriptor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTDescriptor);


impl BluetoothRemoteGATTDescriptor {
    /// Getter of the `characteristic` attribute.
    /// [`BluetoothRemoteGATTDescriptor.characteristic`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor/characteristic)
    pub fn characteristic(&self) -> BluetoothRemoteGATTCharacteristic {
        self.inner.get("characteristic").as_::<BluetoothRemoteGATTCharacteristic>()
    }

}
impl BluetoothRemoteGATTDescriptor {
    /// Getter of the `uuid` attribute.
    /// [`BluetoothRemoteGATTDescriptor.uuid`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor/uuid)
    pub fn uuid(&self) -> Any {
        self.inner.get("uuid").as_::<Any>()
    }

}
impl BluetoothRemoteGATTDescriptor {
    /// Getter of the `value` attribute.
    /// [`BluetoothRemoteGATTDescriptor.value`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor/value)
    pub fn value(&self) -> DataView {
        self.inner.get("value").as_::<DataView>()
    }

}
impl BluetoothRemoteGATTDescriptor {
    /// The readValue method.
    /// [`BluetoothRemoteGATTDescriptor.readValue`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor/readValue)
    pub fn read_value(&self, ) -> Promise<DataView> {
        self.inner.call("readValue", &[]).as_::<Promise<DataView>>()
    }
}
impl BluetoothRemoteGATTDescriptor {
    /// The writeValue method.
    /// [`BluetoothRemoteGATTDescriptor.writeValue`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTDescriptor/writeValue)
    pub fn write_value(&self, value: &Any) -> Promise<Undefined> {
        self.inner.call("writeValue", &[value.into(), ]).as_::<Promise<Undefined>>()
    }
}
