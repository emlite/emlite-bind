use super::*;

/// The USBIsochronousOutTransferPacket class.
/// [`USBIsochronousOutTransferPacket`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBIsochronousOutTransferPacket {
    inner: Any,
}

impl FromVal for USBIsochronousOutTransferPacket {
    fn from_val(v: &Any) -> Self {
        USBIsochronousOutTransferPacket {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBIsochronousOutTransferPacket {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBIsochronousOutTransferPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBIsochronousOutTransferPacket {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBIsochronousOutTransferPacket {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBIsochronousOutTransferPacket> for Any {
    fn from(s: USBIsochronousOutTransferPacket) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBIsochronousOutTransferPacket> for Any {
    fn from(s: &USBIsochronousOutTransferPacket) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBIsochronousOutTransferPacket);

impl USBIsochronousOutTransferPacket {
    /// Getter of the `bytesWritten` attribute.
    /// [`USBIsochronousOutTransferPacket.bytesWritten`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/bytesWritten)
    pub fn bytes_written(&self) -> u32 {
        self.inner.get("bytesWritten").as_::<u32>()
    }
}
impl USBIsochronousOutTransferPacket {
    /// Getter of the `status` attribute.
    /// [`USBIsochronousOutTransferPacket.status`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/status)
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}

impl USBIsochronousOutTransferPacket {
    /// The `new USBIsochronousOutTransferPacket(..)` constructor, creating a new USBIsochronousOutTransferPacket instance
    pub fn new(status: &USBTransferStatus) -> USBIsochronousOutTransferPacket {
        Self {
            inner: Any::global("USBIsochronousOutTransferPacket")
                .new(&[status.into()])
                .as_::<Any>(),
        }
    }
}

impl USBIsochronousOutTransferPacket {
    /// The `new USBIsochronousOutTransferPacket(..)` constructor, creating a new USBIsochronousOutTransferPacket instance
    pub fn new_with_bytes_written(
        status: &USBTransferStatus,
        bytes_written: u32,
    ) -> USBIsochronousOutTransferPacket {
        Self {
            inner: Any::global("USBIsochronousOutTransferPacket")
                .new(&[status.into(), bytes_written.into()])
                .as_::<Any>(),
        }
    }
}
