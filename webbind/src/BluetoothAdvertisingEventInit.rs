use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothAdvertisingEventInit {
    inner: Any,
}
impl FromVal for BluetoothAdvertisingEventInit {
    fn from_val(v: &Any) -> Self {
        BluetoothAdvertisingEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothAdvertisingEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothAdvertisingEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothAdvertisingEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothAdvertisingEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothAdvertisingEventInit> for Any {
    fn from(s: BluetoothAdvertisingEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothAdvertisingEventInit> for Any {
    fn from(s: &BluetoothAdvertisingEventInit) -> Any {
        s.inner.clone()
    }
}

impl BluetoothAdvertisingEventInit {
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }

    pub fn set_device(&mut self, value: &BluetoothDevice) {
        self.inner.set("device", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn uuids(&self) -> TypedArray<Any> {
        self.inner.get("uuids").as_::<TypedArray<Any>>()
    }

    pub fn set_uuids(&mut self, value: &TypedArray<Any>) {
        self.inner.set("uuids", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn appearance(&self) -> u16 {
        self.inner.get("appearance").as_::<u16>()
    }

    pub fn set_appearance(&mut self, value: u16) {
        self.inner.set("appearance", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn tx_power(&self) -> i8 {
        self.inner.get("txPower").as_::<i8>()
    }

    pub fn set_tx_power(&mut self, value: i8) {
        self.inner.set("txPower", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn rssi(&self) -> i8 {
        self.inner.get("rssi").as_::<i8>()
    }

    pub fn set_rssi(&mut self, value: i8) {
        self.inner.set("rssi", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn manufacturer_data(&self) -> BluetoothManufacturerDataMap {
        self.inner
            .get("manufacturerData")
            .as_::<BluetoothManufacturerDataMap>()
    }

    pub fn set_manufacturer_data(&mut self, value: &BluetoothManufacturerDataMap) {
        self.inner.set("manufacturerData", value);
    }
}
impl BluetoothAdvertisingEventInit {
    pub fn service_data(&self) -> BluetoothServiceDataMap {
        self.inner
            .get("serviceData")
            .as_::<BluetoothServiceDataMap>()
    }

    pub fn set_service_data(&mut self, value: &BluetoothServiceDataMap) {
        self.inner.set("serviceData", value);
    }
}
