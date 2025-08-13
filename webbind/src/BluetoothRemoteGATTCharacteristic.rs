use super::*;




/// The BluetoothRemoteGATTCharacteristic class.
/// [`BluetoothRemoteGATTCharacteristic`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTCharacteristic {
    inner: EventTarget,
}

impl FromVal for BluetoothRemoteGATTCharacteristic {
    fn from_val(v: &Any) -> Self {
        BluetoothRemoteGATTCharacteristic { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothRemoteGATTCharacteristic {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothRemoteGATTCharacteristic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothRemoteGATTCharacteristic {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothRemoteGATTCharacteristic {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothRemoteGATTCharacteristic> for Any {
    fn from(s: BluetoothRemoteGATTCharacteristic) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothRemoteGATTCharacteristic> for Any {
    fn from(s: &BluetoothRemoteGATTCharacteristic) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTCharacteristic);


impl BluetoothRemoteGATTCharacteristic {
    /// Getter of the `service` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.service`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/service)
    pub fn service(&self) -> BluetoothRemoteGATTService {
        self.inner.get("service").as_::<BluetoothRemoteGATTService>()
    }

}
impl BluetoothRemoteGATTCharacteristic {
    /// Getter of the `uuid` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.uuid`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/uuid)
    pub fn uuid(&self) -> Any {
        self.inner.get("uuid").as_::<Any>()
    }

}
impl BluetoothRemoteGATTCharacteristic {
    /// Getter of the `properties` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.properties`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/properties)
    pub fn properties(&self) -> BluetoothCharacteristicProperties {
        self.inner.get("properties").as_::<BluetoothCharacteristicProperties>()
    }

}
impl BluetoothRemoteGATTCharacteristic {
    /// Getter of the `value` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.value`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/value)
    pub fn value(&self) -> DataView {
        self.inner.get("value").as_::<DataView>()
    }

}
impl BluetoothRemoteGATTCharacteristic {
    /// The getDescriptor method.
    /// [`BluetoothRemoteGATTCharacteristic.getDescriptor`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptor)
    pub fn get_descriptor(&self, descriptor: &Any) -> Promise<BluetoothRemoteGATTDescriptor> {
        self.inner.call("getDescriptor", &[descriptor.into(), ]).as_::<Promise<BluetoothRemoteGATTDescriptor>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The getDescriptors method.
    /// [`BluetoothRemoteGATTCharacteristic.getDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)
    pub fn get_descriptors0(&self, ) -> Promise<TypedArray<BluetoothRemoteGATTDescriptor>> {
        self.inner.call("getDescriptors", &[]).as_::<Promise<TypedArray<BluetoothRemoteGATTDescriptor>>>()
    }
    /// The getDescriptors method.
    /// [`BluetoothRemoteGATTCharacteristic.getDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)
    pub fn get_descriptors1(&self, descriptor: &Any) -> Promise<TypedArray<BluetoothRemoteGATTDescriptor>> {
        self.inner.call("getDescriptors", &[descriptor.into(), ]).as_::<Promise<TypedArray<BluetoothRemoteGATTDescriptor>>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The readValue method.
    /// [`BluetoothRemoteGATTCharacteristic.readValue`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/readValue)
    pub fn read_value(&self, ) -> Promise<DataView> {
        self.inner.call("readValue", &[]).as_::<Promise<DataView>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The writeValue method.
    /// [`BluetoothRemoteGATTCharacteristic.writeValue`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)
    pub fn write_value(&self, value: &Any) -> Promise<Undefined> {
        self.inner.call("writeValue", &[value.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The writeValueWithResponse method.
    /// [`BluetoothRemoteGATTCharacteristic.writeValueWithResponse`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)
    pub fn write_value_with_response(&self, value: &Any) -> Promise<Undefined> {
        self.inner.call("writeValueWithResponse", &[value.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The writeValueWithoutResponse method.
    /// [`BluetoothRemoteGATTCharacteristic.writeValueWithoutResponse`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)
    pub fn write_value_without_response(&self, value: &Any) -> Promise<Undefined> {
        self.inner.call("writeValueWithoutResponse", &[value.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The startNotifications method.
    /// [`BluetoothRemoteGATTCharacteristic.startNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/startNotifications)
    pub fn start_notifications(&self, ) -> Promise<BluetoothRemoteGATTCharacteristic> {
        self.inner.call("startNotifications", &[]).as_::<Promise<BluetoothRemoteGATTCharacteristic>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// The stopNotifications method.
    /// [`BluetoothRemoteGATTCharacteristic.stopNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/stopNotifications)
    pub fn stop_notifications(&self, ) -> Promise<BluetoothRemoteGATTCharacteristic> {
        self.inner.call("stopNotifications", &[]).as_::<Promise<BluetoothRemoteGATTCharacteristic>>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    /// Getter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    /// Setter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothRemoteGATTCharacteristic.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)
    pub fn set_oncharacteristicvaluechanged(&mut self, value: &Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
