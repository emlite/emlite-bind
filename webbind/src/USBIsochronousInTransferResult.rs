use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct USBIsochronousInTransferResult {
    inner: emlite::Val,
}
impl FromVal for USBIsochronousInTransferResult {
    fn from_val(v: &emlite::Val) -> Self {
        USBIsochronousInTransferResult {
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
impl core::ops::Deref for USBIsochronousInTransferResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBIsochronousInTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBIsochronousInTransferResult> for emlite::Val {
    fn from(s: USBIsochronousInTransferResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBIsochronousInTransferResult {
    pub fn new0(
        packets: jsbind::Sequence<USBIsochronousInTransferPacket>,
    ) -> USBIsochronousInTransferResult {
        Self {
            inner: emlite::Val::global("USBIsochronousInTransferResult")
                .new(&[packets.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(
        packets: jsbind::Sequence<USBIsochronousInTransferPacket>,
        data: jsbind::DataView,
    ) -> USBIsochronousInTransferResult {
        Self {
            inner: emlite::Val::global("USBIsochronousInTransferResult")
                .new(&[packets.into(), data.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBIsochronousInTransferResult {
    pub fn data(&self) -> jsbind::DataView {
        self.inner.get("data").as_::<jsbind::DataView>()
    }
}
impl USBIsochronousInTransferResult {
    pub fn packets(&self) -> jsbind::FrozenArray<USBIsochronousInTransferPacket> {
        self.inner
            .get("packets")
            .as_::<jsbind::FrozenArray<USBIsochronousInTransferPacket>>()
    }
}
