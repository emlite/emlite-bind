use super::*;

/// The BluetoothLEScan class.
/// [`BluetoothLEScan`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScan {
    inner: Any,
}

impl FromVal for BluetoothLEScan {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScan {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothLEScan {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothLEScan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothLEScan {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothLEScan {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothLEScan> for Any {
    fn from(s: BluetoothLEScan) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothLEScan> for Any {
    fn from(s: &BluetoothLEScan) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothLEScan);

impl BluetoothLEScan {
    /// Getter of the `filters` attribute.
    /// [`BluetoothLEScan.filters`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan/filters)
    pub fn filters(&self) -> TypedArray<BluetoothLEScanFilter> {
        self.inner
            .get("filters")
            .as_::<TypedArray<BluetoothLEScanFilter>>()
    }
}
impl BluetoothLEScan {
    /// Getter of the `keepRepeatedDevices` attribute.
    /// [`BluetoothLEScan.keepRepeatedDevices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan/keepRepeatedDevices)
    pub fn keep_repeated_devices(&self) -> bool {
        self.inner.get("keepRepeatedDevices").as_::<bool>()
    }
}
impl BluetoothLEScan {
    /// Getter of the `acceptAllAdvertisements` attribute.
    /// [`BluetoothLEScan.acceptAllAdvertisements`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan/acceptAllAdvertisements)
    pub fn accept_all_advertisements(&self) -> bool {
        self.inner.get("acceptAllAdvertisements").as_::<bool>()
    }
}
impl BluetoothLEScan {
    /// Getter of the `active` attribute.
    /// [`BluetoothLEScan.active`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan/active)
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }
}
impl BluetoothLEScan {
    /// The stop method.
    /// [`BluetoothLEScan.stop`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScan/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
