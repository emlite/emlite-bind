use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BluetoothRemoteGATTServer {
    inner: emlite::Val,
}
impl FromVal for BluetoothRemoteGATTServer {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothRemoteGATTServer {
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
impl core::ops::Deref for BluetoothRemoteGATTServer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothRemoteGATTServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothRemoteGATTServer> for emlite::Val {
    fn from(s: BluetoothRemoteGATTServer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothRemoteGATTServer {
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothRemoteGATTServer {
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl BluetoothRemoteGATTServer {
    pub fn connect(&self) -> jsbind::Promise {
        self.inner.call("connect", &[]).as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTServer {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl BluetoothRemoteGATTServer {
    pub fn get_primary_service(&self, service: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getPrimaryService", &[service.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BluetoothRemoteGATTServer {
    pub fn get_primary_services0(&self) -> jsbind::Promise {
        self.inner
            .call("getPrimaryServices", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_primary_services1(&self, service: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("getPrimaryServices", &[service.into()])
            .as_::<jsbind::Promise>()
    }
}
