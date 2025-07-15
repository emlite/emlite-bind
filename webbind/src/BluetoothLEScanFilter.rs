use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanFilter {
    inner: emlite::Val,
}
impl FromVal for BluetoothLEScanFilter {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothLEScanFilter {
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
impl core::ops::Deref for BluetoothLEScanFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothLEScanFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothLEScanFilter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothLEScanFilter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothLEScanFilter> for emlite::Val {
    fn from(s: BluetoothLEScanFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothLEScanFilter> for emlite::Val {
    fn from(s: &BluetoothLEScanFilter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothLEScanFilter);

impl BluetoothLEScanFilter {
    pub fn new0() -> BluetoothLEScanFilter {
        Self {
            inner: emlite::Val::global("BluetoothLEScanFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: Any) -> BluetoothLEScanFilter {
        Self {
            inner: emlite::Val::global("BluetoothLEScanFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl BluetoothLEScanFilter {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl BluetoothLEScanFilter {
    pub fn name_prefix(&self) -> DOMString {
        self.inner.get("namePrefix").as_::<DOMString>()
    }
}
impl BluetoothLEScanFilter {
    pub fn services(&self) -> FrozenArray<Any> {
        self.inner.get("services").as_::<FrozenArray<Any>>()
    }
}
impl BluetoothLEScanFilter {
    pub fn manufacturer_data(&self) -> BluetoothManufacturerDataFilter {
        self.inner
            .get("manufacturerData")
            .as_::<BluetoothManufacturerDataFilter>()
    }
}
impl BluetoothLEScanFilter {
    pub fn service_data(&self) -> BluetoothServiceDataFilter {
        self.inner
            .get("serviceData")
            .as_::<BluetoothServiceDataFilter>()
    }
}
