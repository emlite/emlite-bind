use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for USBIsochronousInTransferPacket {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBIsochronousInTransferPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBIsochronousInTransferPacket> for emlite::Val {
    fn from(s: USBIsochronousInTransferPacket) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
