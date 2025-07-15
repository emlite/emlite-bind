use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RequestDeviceOptions {
    inner: emlite::Val,
}
impl FromVal for RequestDeviceOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RequestDeviceOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RequestDeviceOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RequestDeviceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RequestDeviceOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RequestDeviceOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<RequestDeviceOptions> for emlite::Val {
    fn from(s: RequestDeviceOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestDeviceOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("filters", value);
    }

}
impl RequestDeviceOptions {
    pub fn exclusion_filters(&self) -> Sequence<Any> {
        self.inner.get("exclusionFilters").as_::<Sequence<Any>>()
    }

    pub fn set_exclusion_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("exclusionFilters", value);
    }

}
impl RequestDeviceOptions {
    pub fn optional_services(&self) -> Sequence<Any> {
        self.inner.get("optionalServices").as_::<Sequence<Any>>()
    }

    pub fn set_optional_services(&mut self, value: Sequence<Any>) {
        self.inner.set("optionalServices", value);
    }

}
impl RequestDeviceOptions {
    pub fn optional_manufacturer_data(&self) -> Sequence<u16> {
        self.inner.get("optionalManufacturerData").as_::<Sequence<u16>>()
    }

    pub fn set_optional_manufacturer_data(&mut self, value: Sequence<u16>) {
        self.inner.set("optionalManufacturerData", value);
    }

}
impl RequestDeviceOptions {
    pub fn accept_all_devices(&self) -> bool {
        self.inner.get("acceptAllDevices").as_::<bool>()
    }

    pub fn set_accept_all_devices(&mut self, value: bool) {
        self.inner.set("acceptAllDevices", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanOptions {
    inner: emlite::Val,
}
impl FromVal for BluetoothLEScanOptions {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothLEScanOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothLEScanOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothLEScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothLEScanOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothLEScanOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<BluetoothLEScanOptions> for emlite::Val {
    fn from(s: BluetoothLEScanOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothLEScanOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("filters", value);
    }

}
impl BluetoothLEScanOptions {
    pub fn keep_repeated_devices(&self) -> bool {
        self.inner.get("keepRepeatedDevices").as_::<bool>()
    }

    pub fn set_keep_repeated_devices(&mut self, value: bool) {
        self.inner.set("keepRepeatedDevices", value);
    }

}
impl BluetoothLEScanOptions {
    pub fn accept_all_advertisements(&self) -> bool {
        self.inner.get("acceptAllAdvertisements").as_::<bool>()
    }

    pub fn set_accept_all_advertisements(&mut self, value: bool) {
        self.inner.set("acceptAllAdvertisements", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Bluetooth {
    inner: EventTarget,
}
impl FromVal for Bluetooth {
    fn from_val(v: &emlite::Val) -> Self {
        Bluetooth { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Bluetooth {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Bluetooth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Bluetooth {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Bluetooth {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Bluetooth> for emlite::Val {
    fn from(s: Bluetooth) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Bluetooth);


impl Bluetooth {
    pub fn get_availability(&self, ) -> Promise {
        self.inner.call("getAvailability", &[]).as_::<Promise>()
    }

}
impl Bluetooth {
    pub fn onavailabilitychanged(&self) -> Any {
        self.inner.get("onavailabilitychanged").as_::<Any>()
    }

    pub fn set_onavailabilitychanged(&mut self, value: Any) {
        self.inner.set("onavailabilitychanged", value);
    }

}
impl Bluetooth {
    pub fn referring_device(&self) -> BluetoothDevice {
        self.inner.get("referringDevice").as_::<BluetoothDevice>()
    }

}
impl Bluetooth {
    pub fn get_devices(&self, ) -> Promise {
        self.inner.call("getDevices", &[]).as_::<Promise>()
    }

}
impl Bluetooth {
    pub fn request_device0(&self, ) -> Promise {
        self.inner.call("requestDevice", &[]).as_::<Promise>()
    }

    pub fn request_device1(&self, options: RequestDeviceOptions) -> Promise {
        self.inner.call("requestDevice", &[options.into(), ]).as_::<Promise>()
    }

}
impl Bluetooth {
    pub fn request_le_scan0(&self, ) -> Promise {
        self.inner.call("requestLEScan", &[]).as_::<Promise>()
    }

    pub fn request_le_scan1(&self, options: BluetoothLEScanOptions) -> Promise {
        self.inner.call("requestLEScan", &[options.into(), ]).as_::<Promise>()
    }

}
impl Bluetooth {
    pub fn onadvertisementreceived(&self) -> Any {
        self.inner.get("onadvertisementreceived").as_::<Any>()
    }

    pub fn set_onadvertisementreceived(&mut self, value: Any) {
        self.inner.set("onadvertisementreceived", value);
    }

}
impl Bluetooth {
    pub fn ongattserverdisconnected(&self) -> Any {
        self.inner.get("ongattserverdisconnected").as_::<Any>()
    }

    pub fn set_ongattserverdisconnected(&mut self, value: Any) {
        self.inner.set("ongattserverdisconnected", value);
    }

}
impl Bluetooth {
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    pub fn set_oncharacteristicvaluechanged(&mut self, value: Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }

}
impl Bluetooth {
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    pub fn set_onserviceadded(&mut self, value: Any) {
        self.inner.set("onserviceadded", value);
    }

}
impl Bluetooth {
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    pub fn set_onservicechanged(&mut self, value: Any) {
        self.inner.set("onservicechanged", value);
    }

}
impl Bluetooth {
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    pub fn set_onserviceremoved(&mut self, value: Any) {
        self.inner.set("onserviceremoved", value);
    }

}
