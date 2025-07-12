use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for BluetoothServiceDataFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BluetoothServiceDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BluetoothServiceDataFilter> for emlite::Val {
    fn from(s: BluetoothServiceDataFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BluetoothServiceDataFilter {
    pub fn new0() -> BluetoothServiceDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothServiceDataFilter")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: jsbind::Object) -> BluetoothServiceDataFilter {
        Self {
            inner: emlite::Val::global("BluetoothServiceDataFilter")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
