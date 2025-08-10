use super::*;

/// The BluetoothDataFilterInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothDataFilterInit {
    inner: Any,
}

impl FromVal for BluetoothDataFilterInit {
    fn from_val(v: &Any) -> Self {
        BluetoothDataFilterInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothDataFilterInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothDataFilterInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothDataFilterInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothDataFilterInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothDataFilterInit> for Any {
    fn from(s: BluetoothDataFilterInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothDataFilterInit> for Any {
    fn from(s: &BluetoothDataFilterInit) -> Any {
        s.inner.clone()
    }
}

impl BluetoothDataFilterInit {
    /// Getter of the `dataPrefix` attribute.
    pub fn data_prefix(&self) -> Any {
        self.inner.get("dataPrefix").as_::<Any>()
    }

    /// Setter of the `dataPrefix` attribute.
    pub fn set_data_prefix(&mut self, value: &Any) {
        self.inner.set("dataPrefix", value);
    }
}
impl BluetoothDataFilterInit {
    /// Getter of the `mask` attribute.
    pub fn mask(&self) -> Any {
        self.inner.get("mask").as_::<Any>()
    }

    /// Setter of the `mask` attribute.
    pub fn set_mask(&mut self, value: &Any) {
        self.inner.set("mask", value);
    }
}
