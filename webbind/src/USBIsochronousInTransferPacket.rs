use super::*;




/// The USBIsochronousInTransferPacket class.
/// [`USBIsochronousInTransferPacket`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBIsochronousInTransferPacket {
    inner: Any,
}

impl FromVal for USBIsochronousInTransferPacket {
    fn from_val(v: &Any) -> Self {
        USBIsochronousInTransferPacket { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBIsochronousInTransferPacket {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBIsochronousInTransferPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBIsochronousInTransferPacket {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBIsochronousInTransferPacket {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<USBIsochronousInTransferPacket> for Any {
    fn from(s: USBIsochronousInTransferPacket) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBIsochronousInTransferPacket> for Any {
    fn from(s: &USBIsochronousInTransferPacket) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBIsochronousInTransferPacket);



impl USBIsochronousInTransferPacket {
    /// The `new USBIsochronousInTransferPacket(..)` constructor, creating a new USBIsochronousInTransferPacket instance
    pub fn new0(status: &USBTransferStatus) -> USBIsochronousInTransferPacket {
        Self {
            inner: Any::global("USBIsochronousInTransferPacket").new(&[status.into()]).as_::<Any>(),
        }
    }

    /// The `new USBIsochronousInTransferPacket(..)` constructor, creating a new USBIsochronousInTransferPacket instance
    pub fn new1(status: &USBTransferStatus, data: &DataView) -> USBIsochronousInTransferPacket {
        Self {
            inner: Any::global("USBIsochronousInTransferPacket").new(&[status.into(), data.into()]).as_::<Any>(),
        }
    }

}
impl USBIsochronousInTransferPacket {
    /// Getter of the `data` attribute.
    /// [`USBIsochronousInTransferPacket.data`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/data)
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }

}
impl USBIsochronousInTransferPacket {
    /// Getter of the `status` attribute.
    /// [`USBIsochronousInTransferPacket.status`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/status)
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }

}
