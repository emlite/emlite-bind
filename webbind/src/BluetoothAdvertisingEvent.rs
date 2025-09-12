use super::*;

/// The BluetoothAdvertisingEvent class.
/// [`BluetoothAdvertisingEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothAdvertisingEvent {
    inner: Event,
}

impl FromVal for BluetoothAdvertisingEvent {
    fn from_val(v: &Any) -> Self {
        BluetoothAdvertisingEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for BluetoothAdvertisingEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothAdvertisingEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothAdvertisingEvent> for Any {
    fn from(s: BluetoothAdvertisingEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothAdvertisingEvent> for Any {
    fn from(s: &BluetoothAdvertisingEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothAdvertisingEvent);

impl BluetoothAdvertisingEvent {
    /// Getter of the `device` attribute.
    /// [`BluetoothAdvertisingEvent.device`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/device)
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `uuids` attribute.
    /// [`BluetoothAdvertisingEvent.uuids`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/uuids)
    pub fn uuids(&self) -> TypedArray<Any> {
        self.inner.get("uuids").as_::<TypedArray<Any>>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `name` attribute.
    /// [`BluetoothAdvertisingEvent.name`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `appearance` attribute.
    /// [`BluetoothAdvertisingEvent.appearance`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/appearance)
    pub fn appearance(&self) -> u16 {
        self.inner.get("appearance").as_::<u16>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `txPower` attribute.
    /// [`BluetoothAdvertisingEvent.txPower`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/txPower)
    pub fn tx_power(&self) -> i8 {
        self.inner.get("txPower").as_::<i8>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `rssi` attribute.
    /// [`BluetoothAdvertisingEvent.rssi`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/rssi)
    pub fn rssi(&self) -> i8 {
        self.inner.get("rssi").as_::<i8>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `manufacturerData` attribute.
    /// [`BluetoothAdvertisingEvent.manufacturerData`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/manufacturerData)
    pub fn manufacturer_data(&self) -> BluetoothManufacturerDataMap {
        self.inner
            .get("manufacturerData")
            .as_::<BluetoothManufacturerDataMap>()
    }
}
impl BluetoothAdvertisingEvent {
    /// Getter of the `serviceData` attribute.
    /// [`BluetoothAdvertisingEvent.serviceData`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/serviceData)
    pub fn service_data(&self) -> BluetoothServiceDataMap {
        self.inner
            .get("serviceData")
            .as_::<BluetoothServiceDataMap>()
    }
}

impl BluetoothAdvertisingEvent {
    /// The `new BluetoothAdvertisingEvent(..)` constructor, creating a new BluetoothAdvertisingEvent instance
    pub fn new(
        type_: &JsString,
        init: &BluetoothAdvertisingEventInit,
    ) -> BluetoothAdvertisingEvent {
        Self {
            inner: Any::global("BluetoothAdvertisingEvent")
                .new(&[type_.into(), init.into()])
                .as_::<Event>(),
        }
    }
}
