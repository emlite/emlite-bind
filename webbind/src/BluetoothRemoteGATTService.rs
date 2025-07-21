use super::*;

/// The BluetoothRemoteGATTService class.
/// [`BluetoothRemoteGATTService`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTService {
    inner: EventTarget,
}
impl FromVal for BluetoothRemoteGATTService {
    fn from_val(v: &Any) -> Self {
        BluetoothRemoteGATTService {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothRemoteGATTService {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothRemoteGATTService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothRemoteGATTService {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothRemoteGATTService {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothRemoteGATTService> for Any {
    fn from(s: BluetoothRemoteGATTService) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothRemoteGATTService> for Any {
    fn from(s: &BluetoothRemoteGATTService) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTService);

impl BluetoothRemoteGATTService {
    /// Getter of the `device` attribute.
    /// [`BluetoothRemoteGATTService.device`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/device)
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `uuid` attribute.
    /// [`BluetoothRemoteGATTService.uuid`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/uuid)
    pub fn uuid(&self) -> Any {
        self.inner.get("uuid").as_::<Any>()
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `isPrimary` attribute.
    /// [`BluetoothRemoteGATTService.isPrimary`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/isPrimary)
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl BluetoothRemoteGATTService {
    /// The getCharacteristic method.
    /// [`BluetoothRemoteGATTService.getCharacteristic`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristic)
    pub fn get_characteristic(
        &self,
        characteristic: &Any,
    ) -> Promise<BluetoothRemoteGATTCharacteristic> {
        self.inner
            .call("getCharacteristic", &[characteristic.into()])
            .as_::<Promise<BluetoothRemoteGATTCharacteristic>>()
    }
}
impl BluetoothRemoteGATTService {
    /// The getCharacteristics method.
    /// [`BluetoothRemoteGATTService.getCharacteristics`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)
    pub fn get_characteristics0(&self) -> Promise<Sequence<BluetoothRemoteGATTCharacteristic>> {
        self.inner
            .call("getCharacteristics", &[])
            .as_::<Promise<Sequence<BluetoothRemoteGATTCharacteristic>>>()
    }
    /// The getCharacteristics method.
    /// [`BluetoothRemoteGATTService.getCharacteristics`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)
    pub fn get_characteristics1(
        &self,
        characteristic: &Any,
    ) -> Promise<Sequence<BluetoothRemoteGATTCharacteristic>> {
        self.inner
            .call("getCharacteristics", &[characteristic.into()])
            .as_::<Promise<Sequence<BluetoothRemoteGATTCharacteristic>>>()
    }
}
impl BluetoothRemoteGATTService {
    /// The getIncludedService method.
    /// [`BluetoothRemoteGATTService.getIncludedService`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedService)
    pub fn get_included_service(&self, service: &Any) -> Promise<BluetoothRemoteGATTService> {
        self.inner
            .call("getIncludedService", &[service.into()])
            .as_::<Promise<BluetoothRemoteGATTService>>()
    }
}
impl BluetoothRemoteGATTService {
    /// The getIncludedServices method.
    /// [`BluetoothRemoteGATTService.getIncludedServices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)
    pub fn get_included_services0(&self) -> Promise<Sequence<BluetoothRemoteGATTService>> {
        self.inner
            .call("getIncludedServices", &[])
            .as_::<Promise<Sequence<BluetoothRemoteGATTService>>>()
    }
    /// The getIncludedServices method.
    /// [`BluetoothRemoteGATTService.getIncludedServices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)
    pub fn get_included_services1(
        &self,
        service: &Any,
    ) -> Promise<Sequence<BluetoothRemoteGATTService>> {
        self.inner
            .call("getIncludedServices", &[service.into()])
            .as_::<Promise<Sequence<BluetoothRemoteGATTService>>>()
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothRemoteGATTService.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    /// Setter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothRemoteGATTService.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)
    pub fn set_oncharacteristicvaluechanged(&mut self, value: &Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `onserviceadded` attribute.
    /// [`BluetoothRemoteGATTService.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    /// Setter of the `onserviceadded` attribute.
    /// [`BluetoothRemoteGATTService.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)
    pub fn set_onserviceadded(&mut self, value: &Any) {
        self.inner.set("onserviceadded", value);
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `onservicechanged` attribute.
    /// [`BluetoothRemoteGATTService.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    /// Setter of the `onservicechanged` attribute.
    /// [`BluetoothRemoteGATTService.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)
    pub fn set_onservicechanged(&mut self, value: &Any) {
        self.inner.set("onservicechanged", value);
    }
}
impl BluetoothRemoteGATTService {
    /// Getter of the `onserviceremoved` attribute.
    /// [`BluetoothRemoteGATTService.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    /// Setter of the `onserviceremoved` attribute.
    /// [`BluetoothRemoteGATTService.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)
    pub fn set_onserviceremoved(&mut self, value: &Any) {
        self.inner.set("onserviceremoved", value);
    }
}
