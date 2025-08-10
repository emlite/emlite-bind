use super::*;

/// The BluetoothManufacturerDataFilterInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothManufacturerDataFilterInit {
    inner: Any,
}

impl FromVal for BluetoothManufacturerDataFilterInit {
    fn from_val(v: &Any) -> Self {
        BluetoothManufacturerDataFilterInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothManufacturerDataFilterInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothManufacturerDataFilterInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothManufacturerDataFilterInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothManufacturerDataFilterInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothManufacturerDataFilterInit> for Any {
    fn from(s: BluetoothManufacturerDataFilterInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothManufacturerDataFilterInit> for Any {
    fn from(s: &BluetoothManufacturerDataFilterInit) -> Any {
        s.inner.clone()
    }
}

impl BluetoothManufacturerDataFilterInit {
    /// Getter of the `companyIdentifier` attribute.
    pub fn company_identifier(&self) -> u16 {
        self.inner.get("companyIdentifier").as_::<u16>()
    }

    /// Setter of the `companyIdentifier` attribute.
    pub fn set_company_identifier(&mut self, value: u16) {
        self.inner.set("companyIdentifier", value);
    }
}
