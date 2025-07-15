use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothAdvertisingEvent {
    inner: Event,
}
impl FromVal for BluetoothAdvertisingEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothAdvertisingEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothAdvertisingEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothAdvertisingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothAdvertisingEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothAdvertisingEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothAdvertisingEvent> for emlite::Val {
    fn from(s: BluetoothAdvertisingEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothAdvertisingEvent);

impl BluetoothAdvertisingEvent {
    pub fn new(type_: DOMString, init: Any) -> BluetoothAdvertisingEvent {
        Self {
            inner: emlite::Val::global("BluetoothAdvertisingEvent")
                .new(&[type_.into(), init.into()])
                .as_::<Event>(),
        }
    }
}
impl BluetoothAdvertisingEvent {
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn uuids(&self) -> FrozenArray<Any> {
        self.inner.get("uuids").as_::<FrozenArray<Any>>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn appearance(&self) -> u16 {
        self.inner.get("appearance").as_::<u16>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn tx_power(&self) -> i8 {
        self.inner.get("txPower").as_::<i8>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn rssi(&self) -> i8 {
        self.inner.get("rssi").as_::<i8>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn manufacturer_data(&self) -> BluetoothManufacturerDataMap {
        self.inner
            .get("manufacturerData")
            .as_::<BluetoothManufacturerDataMap>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn service_data(&self) -> BluetoothServiceDataMap {
        self.inner
            .get("serviceData")
            .as_::<BluetoothServiceDataMap>()
    }
}
