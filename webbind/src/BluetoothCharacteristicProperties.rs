use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothCharacteristicProperties {
    inner: emlite::Val,
}
impl FromVal for BluetoothCharacteristicProperties {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothCharacteristicProperties { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothCharacteristicProperties {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothCharacteristicProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothCharacteristicProperties {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothCharacteristicProperties {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<BluetoothCharacteristicProperties> for emlite::Val {
    fn from(s: BluetoothCharacteristicProperties) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothCharacteristicProperties);


impl BluetoothCharacteristicProperties {
    pub fn broadcast(&self) -> bool {
        self.inner.get("broadcast").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn read(&self) -> bool {
        self.inner.get("read").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn write_without_response(&self) -> bool {
        self.inner.get("writeWithoutResponse").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn write(&self) -> bool {
        self.inner.get("write").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn notify(&self) -> bool {
        self.inner.get("notify").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn indicate(&self) -> bool {
        self.inner.get("indicate").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn authenticated_signed_writes(&self) -> bool {
        self.inner.get("authenticatedSignedWrites").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn reliable_write(&self) -> bool {
        self.inner.get("reliableWrite").as_::<bool>()
    }

}
impl BluetoothCharacteristicProperties {
    pub fn writable_auxiliaries(&self) -> bool {
        self.inner.get("writableAuxiliaries").as_::<bool>()
    }

}
