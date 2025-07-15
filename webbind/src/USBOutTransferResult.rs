use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBOutTransferResult {
    inner: emlite::Val,
}
impl FromVal for USBOutTransferResult {
    fn from_val(v: &emlite::Val) -> Self {
        USBOutTransferResult {
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
impl core::ops::Deref for USBOutTransferResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBOutTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBOutTransferResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBOutTransferResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBOutTransferResult> for emlite::Val {
    fn from(s: USBOutTransferResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&USBOutTransferResult> for emlite::Val {
    fn from(s: &USBOutTransferResult) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBOutTransferResult);

impl USBOutTransferResult {
    pub fn new0(status: &USBTransferStatus) -> USBOutTransferResult {
        Self {
            inner: emlite::Val::global("USBOutTransferResult")
                .new(&[status.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(status: &USBTransferStatus, bytes_written: u32) -> USBOutTransferResult {
        Self {
            inner: emlite::Val::global("USBOutTransferResult")
                .new(&[status.into(), bytes_written.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBOutTransferResult {
    pub fn bytes_written(&self) -> u32 {
        self.inner.get("bytesWritten").as_::<u32>()
    }
}
impl USBOutTransferResult {
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}
