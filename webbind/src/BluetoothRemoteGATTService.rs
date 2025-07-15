use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTService {
    inner: EventTarget,
}
impl FromVal for BluetoothRemoteGATTService {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothRemoteGATTService { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothRemoteGATTService {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothRemoteGATTService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothRemoteGATTService {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothRemoteGATTService {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<BluetoothRemoteGATTService> for emlite::Val {
    fn from(s: BluetoothRemoteGATTService) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTService);


impl BluetoothRemoteGATTService {
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn uuid(&self) -> Any {
        self.inner.get("uuid").as_::<Any>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn get_characteristic(&self, characteristic: Any) -> Promise {
        self.inner.call("getCharacteristic", &[characteristic.into(), ]).as_::<Promise>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn get_characteristics0(&self, ) -> Promise {
        self.inner.call("getCharacteristics", &[]).as_::<Promise>()
    }

    pub fn get_characteristics1(&self, characteristic: Any) -> Promise {
        self.inner.call("getCharacteristics", &[characteristic.into(), ]).as_::<Promise>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn get_included_service(&self, service: Any) -> Promise {
        self.inner.call("getIncludedService", &[service.into(), ]).as_::<Promise>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn get_included_services0(&self, ) -> Promise {
        self.inner.call("getIncludedServices", &[]).as_::<Promise>()
    }

    pub fn get_included_services1(&self, service: Any) -> Promise {
        self.inner.call("getIncludedServices", &[service.into(), ]).as_::<Promise>()
    }

}
impl BluetoothRemoteGATTService {
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    pub fn set_oncharacteristicvaluechanged(&mut self, value: Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }

}
impl BluetoothRemoteGATTService {
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    pub fn set_onserviceadded(&mut self, value: Any) {
        self.inner.set("onserviceadded", value);
    }

}
impl BluetoothRemoteGATTService {
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    pub fn set_onservicechanged(&mut self, value: Any) {
        self.inner.set("onservicechanged", value);
    }

}
impl BluetoothRemoteGATTService {
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    pub fn set_onserviceremoved(&mut self, value: Any) {
        self.inner.set("onserviceremoved", value);
    }

}
