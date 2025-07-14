use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTCharacteristic {
    inner: EventTarget,
}
impl FromVal for BluetoothRemoteGATTCharacteristic {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothRemoteGATTCharacteristic {
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
impl core::ops::Deref for BluetoothRemoteGATTCharacteristic {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothRemoteGATTCharacteristic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothRemoteGATTCharacteristic {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothRemoteGATTCharacteristic {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothRemoteGATTCharacteristic> for emlite::Val {
    fn from(s: BluetoothRemoteGATTCharacteristic) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTCharacteristic);

impl BluetoothRemoteGATTCharacteristic {
    pub fn service(&self) -> BluetoothRemoteGATTService {
        self.inner
            .get("service")
            .as_::<BluetoothRemoteGATTService>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn uuid(&self) -> jsbind::Any {
        self.inner.get("uuid").as_::<jsbind::Any>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn properties(&self) -> BluetoothCharacteristicProperties {
        self.inner
            .get("properties")
            .as_::<BluetoothCharacteristicProperties>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn value(&self) -> jsbind::DataView {
        self.inner.get("value").as_::<jsbind::DataView>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn get_descriptor(&self, descriptor: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getDescriptor", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn get_descriptors0(&self) -> jsbind::Promise {
        self.inner
            .call("getDescriptors", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_descriptors1(&self, descriptor: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getDescriptors", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn read_value(&self) -> jsbind::Promise {
        self.inner.call("readValue", &[]).as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn write_value(&self, value: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("writeValue", &[value.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn write_value_with_response(&self, value: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("writeValueWithResponse", &[value.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn write_value_without_response(&self, value: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("writeValueWithoutResponse", &[value.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn start_notifications(&self) -> jsbind::Promise {
        self.inner
            .call("startNotifications", &[])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn stop_notifications(&self) -> jsbind::Promise {
        self.inner
            .call("stopNotifications", &[])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTCharacteristic {
    pub fn oncharacteristicvaluechanged(&self) -> jsbind::Any {
        self.inner
            .get("oncharacteristicvaluechanged")
            .as_::<jsbind::Any>()
    }

    pub fn set_oncharacteristicvaluechanged(&mut self, value: jsbind::Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
