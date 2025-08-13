use super::*;




/// The USBIsochronousInTransferResult class.
/// [`USBIsochronousInTransferResult`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBIsochronousInTransferResult {
    inner: Any,
}

impl FromVal for USBIsochronousInTransferResult {
    fn from_val(v: &Any) -> Self {
        USBIsochronousInTransferResult { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBIsochronousInTransferResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBIsochronousInTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBIsochronousInTransferResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBIsochronousInTransferResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<USBIsochronousInTransferResult> for Any {
    fn from(s: USBIsochronousInTransferResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBIsochronousInTransferResult> for Any {
    fn from(s: &USBIsochronousInTransferResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBIsochronousInTransferResult);



impl USBIsochronousInTransferResult {
    /// The `new USBIsochronousInTransferResult(..)` constructor, creating a new USBIsochronousInTransferResult instance
    pub fn new0(packets: &TypedArray<USBIsochronousInTransferPacket>) -> USBIsochronousInTransferResult {
        Self {
            inner: Any::global("USBIsochronousInTransferResult").new(&[packets.into()]).as_::<Any>(),
        }
    }

    /// The `new USBIsochronousInTransferResult(..)` constructor, creating a new USBIsochronousInTransferResult instance
    pub fn new1(packets: &TypedArray<USBIsochronousInTransferPacket>, data: &DataView) -> USBIsochronousInTransferResult {
        Self {
            inner: Any::global("USBIsochronousInTransferResult").new(&[packets.into(), data.into()]).as_::<Any>(),
        }
    }

}
impl USBIsochronousInTransferResult {
    /// Getter of the `data` attribute.
    /// [`USBIsochronousInTransferResult.data`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/data)
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }

}
impl USBIsochronousInTransferResult {
    /// Getter of the `packets` attribute.
    /// [`USBIsochronousInTransferResult.packets`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/packets)
    pub fn packets(&self) -> TypedArray<USBIsochronousInTransferPacket> {
        self.inner.get("packets").as_::<TypedArray<USBIsochronousInTransferPacket>>()
    }

}
