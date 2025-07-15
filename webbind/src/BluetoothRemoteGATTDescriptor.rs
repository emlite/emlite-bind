use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTDescriptor {
    inner: emlite::Val,
}
impl FromVal for BluetoothRemoteGATTDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothRemoteGATTDescriptor {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothRemoteGATTDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothRemoteGATTDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothRemoteGATTDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothRemoteGATTDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothRemoteGATTDescriptor> for emlite::Val {
    fn from(s: BluetoothRemoteGATTDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTDescriptor);

impl BluetoothRemoteGATTDescriptor {
    pub fn characteristic(&self) -> BluetoothRemoteGATTCharacteristic {
        self.inner
            .get("characteristic")
            .as_::<BluetoothRemoteGATTCharacteristic>()
    }
}
impl BluetoothRemoteGATTDescriptor {
    pub fn uuid(&self) -> Any {
        self.inner.get("uuid").as_::<Any>()
    }
}
impl BluetoothRemoteGATTDescriptor {
    pub fn value(&self) -> DataView {
        self.inner.get("value").as_::<DataView>()
    }
}
impl BluetoothRemoteGATTDescriptor {
    pub fn read_value(&self) -> Promise {
        self.inner.call("readValue", &[]).as_::<Promise>()
    }
}
impl BluetoothRemoteGATTDescriptor {
    pub fn write_value(&self, value: Any) -> Promise {
        self.inner
            .call("writeValue", &[value.into()])
            .as_::<Promise>()
    }
}
