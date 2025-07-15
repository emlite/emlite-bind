use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for BluetoothUUID {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothUUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothUUID {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothUUID {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothUUID> for emlite::Val {
    fn from(s: BluetoothUUID) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothUUID);

impl BluetoothUUID {
    pub fn get_service(name: Any) -> Any {
        emlite::Val::global("BluetoothUUID")
            .call("getService", &[name.into()])
            .as_::<Any>()
    }
}
impl BluetoothUUID {
    pub fn get_characteristic(name: Any) -> Any {
        emlite::Val::global("BluetoothUUID")
            .call("getCharacteristic", &[name.into()])
            .as_::<Any>()
    }
}
impl BluetoothUUID {
    pub fn get_descriptor(name: Any) -> Any {
        emlite::Val::global("BluetoothUUID")
            .call("getDescriptor", &[name.into()])
            .as_::<Any>()
    }
}
impl BluetoothUUID {
    pub fn canonical_uuid(alias: u32) -> Any {
        emlite::Val::global("BluetoothUUID")
            .call("canonicalUUID", &[alias.into()])
            .as_::<Any>()
    }
}
