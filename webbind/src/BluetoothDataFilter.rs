use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothDataFilter {
    inner: emlite::Val,
}
impl FromVal for BluetoothDataFilter {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothDataFilter {
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
impl core::ops::Deref for BluetoothDataFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BluetoothDataFilter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothDataFilter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothDataFilter> for emlite::Val {
    fn from(s: BluetoothDataFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothDataFilter> for emlite::Val {
    fn from(s: &BluetoothDataFilter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothDataFilter);

impl BluetoothDataFilter {
    pub fn new0() -> BluetoothDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothDataFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: &Any) -> BluetoothDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothDataFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl BluetoothDataFilter {
    pub fn data_prefix(&self) -> ArrayBuffer {
        self.inner.get("dataPrefix").as_::<ArrayBuffer>()
    }
}
impl BluetoothDataFilter {
    pub fn mask(&self) -> ArrayBuffer {
        self.inner.get("mask").as_::<ArrayBuffer>()
    }
}
