use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanFilterInit {
    inner: Any,
}
impl FromVal for BluetoothLEScanFilterInit {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScanFilterInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothLEScanFilterInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothLEScanFilterInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothLEScanFilterInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothLEScanFilterInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothLEScanFilterInit> for Any {
    fn from(s: BluetoothLEScanFilterInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothLEScanFilterInit> for Any {
    fn from(s: &BluetoothLEScanFilterInit) -> Any {
        s.inner.clone()
    }
}

impl BluetoothLEScanFilterInit {
    pub fn services(&self) -> TypedArray<Any> {
        self.inner.get("services").as_::<TypedArray<Any>>()
    }

    pub fn set_services(&mut self, value: &TypedArray<Any>) {
        self.inner.set("services", value);
    }
}
impl BluetoothLEScanFilterInit {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl BluetoothLEScanFilterInit {
    pub fn name_prefix(&self) -> JsString {
        self.inner.get("namePrefix").as_::<JsString>()
    }

    pub fn set_name_prefix(&mut self, value: &JsString) {
        self.inner.set("namePrefix", value);
    }
}
impl BluetoothLEScanFilterInit {
    pub fn manufacturer_data(&self) -> TypedArray<BluetoothManufacturerDataFilterInit> {
        self.inner
            .get("manufacturerData")
            .as_::<TypedArray<BluetoothManufacturerDataFilterInit>>()
    }

    pub fn set_manufacturer_data(
        &mut self,
        value: &TypedArray<BluetoothManufacturerDataFilterInit>,
    ) {
        self.inner.set("manufacturerData", value);
    }
}
impl BluetoothLEScanFilterInit {
    pub fn service_data(&self) -> TypedArray<BluetoothServiceDataFilterInit> {
        self.inner
            .get("serviceData")
            .as_::<TypedArray<BluetoothServiceDataFilterInit>>()
    }

    pub fn set_service_data(&mut self, value: &TypedArray<BluetoothServiceDataFilterInit>) {
        self.inner.set("serviceData", value);
    }
}
