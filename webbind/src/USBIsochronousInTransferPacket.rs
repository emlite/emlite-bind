use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBIsochronousInTransferPacket {
    inner: emlite::Val,
}
impl FromVal for USBIsochronousInTransferPacket {
    fn from_val(v: &emlite::Val) -> Self {
        USBIsochronousInTransferPacket {
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
impl core::ops::Deref for USBIsochronousInTransferPacket {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBIsochronousInTransferPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBIsochronousInTransferPacket {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBIsochronousInTransferPacket {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBIsochronousInTransferPacket> for emlite::Val {
    fn from(s: USBIsochronousInTransferPacket) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBIsochronousInTransferPacket);

impl USBIsochronousInTransferPacket {
    pub fn new0(status: USBTransferStatus) -> USBIsochronousInTransferPacket {
        Self {
            inner: emlite::Val::global("USBIsochronousInTransferPacket")
                .new(&[status.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(
        status: USBTransferStatus,
        data: jsbind::DataView,
    ) -> USBIsochronousInTransferPacket {
        Self {
            inner: emlite::Val::global("USBIsochronousInTransferPacket")
                .new(&[status.into(), data.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBIsochronousInTransferPacket {
    pub fn data(&self) -> jsbind::DataView {
        self.inner.get("data").as_::<jsbind::DataView>()
    }
}
impl USBIsochronousInTransferPacket {
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}
