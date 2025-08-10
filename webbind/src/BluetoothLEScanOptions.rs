use super::*;

/// The BluetoothLEScanOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanOptions {
    inner: Any,
}

impl FromVal for BluetoothLEScanOptions {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScanOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothLEScanOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothLEScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothLEScanOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothLEScanOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothLEScanOptions> for Any {
    fn from(s: BluetoothLEScanOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothLEScanOptions> for Any {
    fn from(s: &BluetoothLEScanOptions) -> Any {
        s.inner.clone()
    }
}

impl BluetoothLEScanOptions {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<BluetoothLEScanFilterInit> {
        self.inner
            .get("filters")
            .as_::<TypedArray<BluetoothLEScanFilterInit>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<BluetoothLEScanFilterInit>) {
        self.inner.set("filters", value);
    }
}
impl BluetoothLEScanOptions {
    /// Getter of the `keepRepeatedDevices` attribute.
    pub fn keep_repeated_devices(&self) -> bool {
        self.inner.get("keepRepeatedDevices").as_::<bool>()
    }

    /// Setter of the `keepRepeatedDevices` attribute.
    pub fn set_keep_repeated_devices(&mut self, value: bool) {
        self.inner.set("keepRepeatedDevices", value);
    }
}
impl BluetoothLEScanOptions {
    /// Getter of the `acceptAllAdvertisements` attribute.
    pub fn accept_all_advertisements(&self) -> bool {
        self.inner.get("acceptAllAdvertisements").as_::<bool>()
    }

    /// Setter of the `acceptAllAdvertisements` attribute.
    pub fn set_accept_all_advertisements(&mut self, value: bool) {
        self.inner.set("acceptAllAdvertisements", value);
    }
}
