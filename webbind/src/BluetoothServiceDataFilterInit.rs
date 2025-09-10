use super::*;

/// The BluetoothServiceDataFilterInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothServiceDataFilterInit {
    inner: Any,
}

impl FromVal for BluetoothServiceDataFilterInit {
    fn from_val(v: &Any) -> Self {
        BluetoothServiceDataFilterInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothServiceDataFilterInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothServiceDataFilterInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothServiceDataFilterInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothServiceDataFilterInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothServiceDataFilterInit> for Any {
    fn from(s: BluetoothServiceDataFilterInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothServiceDataFilterInit> for Any {
    fn from(s: &BluetoothServiceDataFilterInit) -> Any {
        s.inner.clone()
    }
}

impl BluetoothServiceDataFilterInit {
    /// Getter of the `service` attribute.
    pub fn service(&self) -> Any {
        self.inner.get("service").as_::<Any>()
    }

    /// Setter of the `service` attribute.
    pub fn set_service(&mut self, value: &Any) {
        self.inner.set("service", value);
    }
}
