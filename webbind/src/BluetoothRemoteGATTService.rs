use super::*;

#[derive(Clone, Debug)]
pub struct BluetoothRemoteGATTService {
    inner: EventTarget,
}
impl FromVal for BluetoothRemoteGATTService {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothRemoteGATTService {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BluetoothRemoteGATTService {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothRemoteGATTService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothRemoteGATTService> for emlite::Val {
    fn from(s: BluetoothRemoteGATTService) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothRemoteGATTService {
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn uuid(&self) -> jsbind::Any {
        self.inner.get("uuid").as_::<jsbind::Any>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn get_characteristic(&self, characteristic: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getCharacteristic", &[characteristic.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn get_characteristics0(&self) -> jsbind::Promise {
        self.inner
            .call("getCharacteristics", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_characteristics1(&self, characteristic: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getCharacteristics", &[characteristic.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn get_included_service(&self, service: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getIncludedService", &[service.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn get_included_services0(&self) -> jsbind::Promise {
        self.inner
            .call("getIncludedServices", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_included_services1(&self, service: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getIncludedServices", &[service.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTService {
    pub fn oncharacteristicvaluechanged(&self) -> jsbind::Any {
        self.inner
            .get("oncharacteristicvaluechanged")
            .as_::<jsbind::Any>()
    }

    pub fn set_oncharacteristicvaluechanged(&mut self, value: jsbind::Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
impl BluetoothRemoteGATTService {
    pub fn onserviceadded(&self) -> jsbind::Any {
        self.inner.get("onserviceadded").as_::<jsbind::Any>()
    }

    pub fn set_onserviceadded(&mut self, value: jsbind::Any) {
        self.inner.set("onserviceadded", value);
    }
}
impl BluetoothRemoteGATTService {
    pub fn onservicechanged(&self) -> jsbind::Any {
        self.inner.get("onservicechanged").as_::<jsbind::Any>()
    }

    pub fn set_onservicechanged(&mut self, value: jsbind::Any) {
        self.inner.set("onservicechanged", value);
    }
}
impl BluetoothRemoteGATTService {
    pub fn onserviceremoved(&self) -> jsbind::Any {
        self.inner.get("onserviceremoved").as_::<jsbind::Any>()
    }

    pub fn set_onserviceremoved(&mut self, value: jsbind::Any) {
        self.inner.set("onserviceremoved", value);
    }
}
