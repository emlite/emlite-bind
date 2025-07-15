use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for BluetoothManufacturerDataFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothManufacturerDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothManufacturerDataFilter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothManufacturerDataFilter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothManufacturerDataFilter> for emlite::Val {
    fn from(s: BluetoothManufacturerDataFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothManufacturerDataFilter> for emlite::Val {
    fn from(s: &BluetoothManufacturerDataFilter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothManufacturerDataFilter);

impl BluetoothManufacturerDataFilter {
    pub fn new0() -> BluetoothManufacturerDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothManufacturerDataFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: &Object) -> BluetoothManufacturerDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothManufacturerDataFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
