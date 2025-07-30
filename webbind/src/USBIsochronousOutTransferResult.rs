use super::*;

/// The USBIsochronousOutTransferResult class.
/// [`USBIsochronousOutTransferResult`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBIsochronousOutTransferResult {
    inner: Any,
}
impl FromVal for USBIsochronousOutTransferResult {
    fn from_val(v: &Any) -> Self {
        USBIsochronousOutTransferResult {
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
impl core::ops::Deref for USBIsochronousOutTransferResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBIsochronousOutTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBIsochronousOutTransferResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBIsochronousOutTransferResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBIsochronousOutTransferResult> for Any {
    fn from(s: USBIsochronousOutTransferResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBIsochronousOutTransferResult> for Any {
    fn from(s: &USBIsochronousOutTransferResult) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBIsochronousOutTransferResult);

impl USBIsochronousOutTransferResult {
    /// The `new USBIsochronousOutTransferResult(..)` constructor, creating a new USBIsochronousOutTransferResult instance
    pub fn new(
        packets: &TypedArray<USBIsochronousOutTransferPacket>,
    ) -> USBIsochronousOutTransferResult {
        Self {
            inner: Any::global("USBIsochronousOutTransferResult")
                .new(&[packets.into()])
                .as_::<Any>(),
        }
    }
}
impl USBIsochronousOutTransferResult {
    /// Getter of the `packets` attribute.
    /// [`USBIsochronousOutTransferResult.packets`](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult/packets)
    pub fn packets(&self) -> TypedArray<USBIsochronousOutTransferPacket> {
        self.inner
            .get("packets")
            .as_::<TypedArray<USBIsochronousOutTransferPacket>>()
    }
}
