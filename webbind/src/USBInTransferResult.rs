use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBInTransferResult {
    inner: emlite::Val,
}
impl FromVal for USBInTransferResult {
    fn from_val(v: &emlite::Val) -> Self {
        USBInTransferResult {
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
impl core::ops::Deref for USBInTransferResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBInTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBInTransferResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBInTransferResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBInTransferResult> for emlite::Val {
    fn from(s: USBInTransferResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBInTransferResult);

impl USBInTransferResult {
    pub fn new0(status: USBTransferStatus) -> USBInTransferResult {
        Self {
            inner: emlite::Val::global("USBInTransferResult")
                .new(&[status.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(status: USBTransferStatus, data: DataView) -> USBInTransferResult {
        Self {
            inner: emlite::Val::global("USBInTransferResult")
                .new(&[status.into(), data.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBInTransferResult {
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }
}
impl USBInTransferResult {
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}
