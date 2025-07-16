use super::*;

/// The USBOutTransferResult class.
/// [`USBOutTransferResult`](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBOutTransferResult {
    inner: Any,
}
impl FromVal for USBOutTransferResult {
    fn from_val(v: &Any) -> Self {
        USBOutTransferResult {
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
impl core::ops::Deref for USBOutTransferResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBOutTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBOutTransferResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBOutTransferResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBOutTransferResult> for Any {
    fn from(s: USBOutTransferResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBOutTransferResult> for Any {
    fn from(s: &USBOutTransferResult) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBOutTransferResult);

impl USBOutTransferResult {
    /// The `new USBOutTransferResult(..)` constructor, creating a new USBOutTransferResult instance
    pub fn new0(status: &USBTransferStatus) -> USBOutTransferResult {
        Self {
            inner: Any::global("USBOutTransferResult")
                .new(&[status.into()])
                .as_::<Any>(),
        }
    }

    /// The `new USBOutTransferResult(..)` constructor, creating a new USBOutTransferResult instance
    pub fn new1(status: &USBTransferStatus, bytes_written: u32) -> USBOutTransferResult {
        Self {
            inner: Any::global("USBOutTransferResult")
                .new(&[status.into(), bytes_written.into()])
                .as_::<Any>(),
        }
    }
}
impl USBOutTransferResult {
    /// Getter of the `bytesWritten` attribute.
    /// [`USBOutTransferResult.bytesWritten`](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/bytesWritten)
    pub fn bytes_written(&self) -> u32 {
        self.inner.get("bytesWritten").as_::<u32>()
    }
}
impl USBOutTransferResult {
    /// Getter of the `status` attribute.
    /// [`USBOutTransferResult.status`](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/status)
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}
