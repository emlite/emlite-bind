use super::*;




/// The BluetoothCharacteristicProperties class.
/// [`BluetoothCharacteristicProperties`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothCharacteristicProperties {
    inner: Any,
}

impl FromVal for BluetoothCharacteristicProperties {
    fn from_val(v: &Any) -> Self {
        BluetoothCharacteristicProperties { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothCharacteristicProperties {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothCharacteristicProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothCharacteristicProperties {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothCharacteristicProperties {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothCharacteristicProperties> for Any {
    fn from(s: BluetoothCharacteristicProperties) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothCharacteristicProperties> for Any {
    fn from(s: &BluetoothCharacteristicProperties) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothCharacteristicProperties);


impl BluetoothCharacteristicProperties {
    /// Getter of the `broadcast` attribute.
    /// [`BluetoothCharacteristicProperties.broadcast`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/broadcast)
    pub fn broadcast(&self) -> bool {
        self.inner.get("broadcast").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `read` attribute.
    /// [`BluetoothCharacteristicProperties.read`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/read)
    pub fn read(&self) -> bool {
        self.inner.get("read").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `writeWithoutResponse` attribute.
    /// [`BluetoothCharacteristicProperties.writeWithoutResponse`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/writeWithoutResponse)
    pub fn write_without_response(&self) -> bool {
        self.inner.get("writeWithoutResponse").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `write` attribute.
    /// [`BluetoothCharacteristicProperties.write`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/write)
    pub fn write(&self) -> bool {
        self.inner.get("write").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `notify` attribute.
    /// [`BluetoothCharacteristicProperties.notify`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/notify)
    pub fn notify(&self) -> bool {
        self.inner.get("notify").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `indicate` attribute.
    /// [`BluetoothCharacteristicProperties.indicate`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/indicate)
    pub fn indicate(&self) -> bool {
        self.inner.get("indicate").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `authenticatedSignedWrites` attribute.
    /// [`BluetoothCharacteristicProperties.authenticatedSignedWrites`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/authenticatedSignedWrites)
    pub fn authenticated_signed_writes(&self) -> bool {
        self.inner.get("authenticatedSignedWrites").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `reliableWrite` attribute.
    /// [`BluetoothCharacteristicProperties.reliableWrite`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/reliableWrite)
    pub fn reliable_write(&self) -> bool {
        self.inner.get("reliableWrite").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    /// Getter of the `writableAuxiliaries` attribute.
    /// [`BluetoothCharacteristicProperties.writableAuxiliaries`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/writableAuxiliaries)
    pub fn writable_auxiliaries(&self) -> bool {
        self.inner.get("writableAuxiliaries").as_::<bool>()
    }

}
