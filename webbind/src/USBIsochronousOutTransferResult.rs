use super::*;

#[derive(Clone, Debug)]
pub struct USBIsochronousOutTransferResult {
    inner: emlite::Val,
}
impl FromVal for USBIsochronousOutTransferResult {
    fn from_val(v: &emlite::Val) -> Self {
        USBIsochronousOutTransferResult {
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
impl std::ops::Deref for USBIsochronousOutTransferResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBIsochronousOutTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBIsochronousOutTransferResult> for emlite::Val {
    fn from(s: USBIsochronousOutTransferResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBIsochronousOutTransferResult {
    pub fn new(
        packets: jsbind::Sequence<USBIsochronousOutTransferPacket>,
    ) -> USBIsochronousOutTransferResult {
        Self {
            inner: emlite::Val::global("USBIsochronousOutTransferResult")
                .new(&[packets.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBIsochronousOutTransferResult {
    pub fn packets(&self) -> jsbind::FrozenArray<USBIsochronousOutTransferPacket> {
        self.inner
            .get("packets")
            .as_::<jsbind::FrozenArray<USBIsochronousOutTransferPacket>>()
    }
}
