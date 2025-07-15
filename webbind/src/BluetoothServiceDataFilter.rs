use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothServiceDataFilter {
    inner: emlite::Val,
}
impl FromVal for BluetoothServiceDataFilter {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothServiceDataFilter {
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
impl core::ops::Deref for BluetoothServiceDataFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothServiceDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothServiceDataFilter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothServiceDataFilter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothServiceDataFilter> for emlite::Val {
    fn from(s: BluetoothServiceDataFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothServiceDataFilter> for emlite::Val {
    fn from(s: &BluetoothServiceDataFilter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothServiceDataFilter);

impl BluetoothServiceDataFilter {
    pub fn new0() -> BluetoothServiceDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothServiceDataFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: &Object) -> BluetoothServiceDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothServiceDataFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
