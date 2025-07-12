use super::*;

#[derive(Clone, Debug)]
pub struct BluetoothLEScan {
    inner: emlite::Val,
}
impl FromVal for BluetoothLEScan {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothLEScan {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BluetoothLEScan {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothLEScan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothLEScan> for emlite::Val {
    fn from(s: BluetoothLEScan) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothLEScan {
    pub fn filters(&self) -> jsbind::FrozenArray<BluetoothLEScanFilter> {
        self.inner
            .get("filters")
            .as_::<jsbind::FrozenArray<BluetoothLEScanFilter>>()
    }
}
impl BluetoothLEScan {
    pub fn keep_repeated_devices(&self) -> bool {
        self.inner.get("keepRepeatedDevices").as_::<bool>()
    }
}
impl BluetoothLEScan {
    pub fn accept_all_advertisements(&self) -> bool {
        self.inner.get("acceptAllAdvertisements").as_::<bool>()
    }
}
impl BluetoothLEScan {
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }
}
impl BluetoothLEScan {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
