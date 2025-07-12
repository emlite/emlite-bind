use super::*;

#[derive(Clone, Debug)]
pub struct BluetoothUUID {
    inner: emlite::Val,
}
impl FromVal for BluetoothUUID {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothUUID {
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
impl std::ops::Deref for BluetoothUUID {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothUUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothUUID> for emlite::Val {
    fn from(s: BluetoothUUID) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothUUID {
    pub fn get_service(name: jsbind::Any) -> jsbind::Any {
        emlite::Val::global("bluetoothuuid")
            .call("getService", &[name.into()])
            .as_::<jsbind::Any>()
    }
}
impl BluetoothUUID {
    pub fn get_characteristic(name: jsbind::Any) -> jsbind::Any {
        emlite::Val::global("bluetoothuuid")
            .call("getCharacteristic", &[name.into()])
            .as_::<jsbind::Any>()
    }
}
impl BluetoothUUID {
    pub fn get_descriptor(name: jsbind::Any) -> jsbind::Any {
        emlite::Val::global("bluetoothuuid")
            .call("getDescriptor", &[name.into()])
            .as_::<jsbind::Any>()
    }
}
impl BluetoothUUID {
    pub fn canonical_uuid(alias: u32) -> jsbind::Any {
        emlite::Val::global("bluetoothuuid")
            .call("canonicalUUID", &[alias.into()])
            .as_::<jsbind::Any>()
    }
}
