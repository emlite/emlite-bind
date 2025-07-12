use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for BluetoothAdvertisingEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothAdvertisingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothAdvertisingEvent> for emlite::Val {
    fn from(s: BluetoothAdvertisingEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothAdvertisingEvent {
    pub fn new(type_: jsbind::DOMString, init: jsbind::Any) -> BluetoothAdvertisingEvent {
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
    pub fn uuids(&self) -> jsbind::FrozenArray<jsbind::Any> {
        self.inner
            .get("uuids")
            .as_::<jsbind::FrozenArray<jsbind::Any>>()
    }
}
impl BluetoothAdvertisingEvent {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
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
