use super::*;




/// The RequestDeviceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RequestDeviceOptions {
    inner: Any,
}

impl FromVal for RequestDeviceOptions {
    fn from_val(v: &Any) -> Self {
        RequestDeviceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RequestDeviceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RequestDeviceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RequestDeviceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RequestDeviceOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RequestDeviceOptions> for Any {
    fn from(s: RequestDeviceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RequestDeviceOptions> for Any {
    fn from(s: &RequestDeviceOptions) -> Any {
        s.inner.clone()
    }
}

impl RequestDeviceOptions {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<BluetoothLEScanFilterInit> {
        self.inner.get("filters").as_::<TypedArray<BluetoothLEScanFilterInit>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<BluetoothLEScanFilterInit>) {
        self.inner.set("filters", value);
    }
}
impl RequestDeviceOptions {
    /// Getter of the `exclusionFilters` attribute.
    pub fn exclusion_filters(&self) -> TypedArray<BluetoothLEScanFilterInit> {
        self.inner.get("exclusionFilters").as_::<TypedArray<BluetoothLEScanFilterInit>>()
    }

    /// Setter of the `exclusionFilters` attribute.
    pub fn set_exclusion_filters(&mut self, value: &TypedArray<BluetoothLEScanFilterInit>) {
        self.inner.set("exclusionFilters", value);
    }
}
impl RequestDeviceOptions {
    /// Getter of the `optionalServices` attribute.
    pub fn optional_services(&self) -> TypedArray<Any> {
        self.inner.get("optionalServices").as_::<TypedArray<Any>>()
    }

    /// Setter of the `optionalServices` attribute.
    pub fn set_optional_services(&mut self, value: &TypedArray<Any>) {
        self.inner.set("optionalServices", value);
    }
}
impl RequestDeviceOptions {
    /// Getter of the `optionalManufacturerData` attribute.
    pub fn optional_manufacturer_data(&self) -> TypedArray<u16> {
        self.inner.get("optionalManufacturerData").as_::<TypedArray<u16>>()
    }

    /// Setter of the `optionalManufacturerData` attribute.
    pub fn set_optional_manufacturer_data(&mut self, value: TypedArray<u16>) {
        self.inner.set("optionalManufacturerData", value);
    }
}
impl RequestDeviceOptions {
    /// Getter of the `acceptAllDevices` attribute.
    pub fn accept_all_devices(&self) -> bool {
        self.inner.get("acceptAllDevices").as_::<bool>()
    }

    /// Setter of the `acceptAllDevices` attribute.
    pub fn set_accept_all_devices(&mut self, value: bool) {
        self.inner.set("acceptAllDevices", value);
    }
}
