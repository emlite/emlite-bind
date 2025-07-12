use super::*;

#[derive(Clone, Debug)]
pub struct BluetoothManufacturerDataFilter {
    inner: emlite::Val,
}
impl FromVal for BluetoothManufacturerDataFilter {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothManufacturerDataFilter {
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
impl std::ops::Deref for BluetoothManufacturerDataFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothManufacturerDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothManufacturerDataFilter> for emlite::Val {
    fn from(s: BluetoothManufacturerDataFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothManufacturerDataFilter {
    pub fn new0() -> BluetoothManufacturerDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothManufacturerDataFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: jsbind::Object) -> BluetoothManufacturerDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothManufacturerDataFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
