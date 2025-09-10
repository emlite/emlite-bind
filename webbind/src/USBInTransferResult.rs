use super::*;

/// The USBInTransferResult class.
/// [`USBInTransferResult`](https://developer.mozilla.org/en-US/docs/Web/API/USBInTransferResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBInTransferResult {
    inner: Any,
}

impl FromVal for USBInTransferResult {
    fn from_val(v: &Any) -> Self {
        USBInTransferResult {
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

impl core::ops::Deref for USBInTransferResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBInTransferResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBInTransferResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBInTransferResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBInTransferResult> for Any {
    fn from(s: USBInTransferResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBInTransferResult> for Any {
    fn from(s: &USBInTransferResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBInTransferResult);

impl USBInTransferResult {
    /// The `new USBInTransferResult(..)` constructor, creating a new USBInTransferResult instance
    pub fn new0(status: &USBTransferStatus) -> USBInTransferResult {
        Self {
            inner: Any::global("USBInTransferResult")
                .new(&[status.into()])
                .as_::<Any>(),
        }
    }

    /// The `new USBInTransferResult(..)` constructor, creating a new USBInTransferResult instance
    pub fn new1(status: &USBTransferStatus, data: &DataView) -> USBInTransferResult {
        Self {
            inner: Any::global("USBInTransferResult")
                .new(&[status.into(), data.into()])
                .as_::<Any>(),
        }
    }
}
impl USBInTransferResult {
    /// Getter of the `data` attribute.
    /// [`USBInTransferResult.data`](https://developer.mozilla.org/en-US/docs/Web/API/USBInTransferResult/data)
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }
}
impl USBInTransferResult {
    /// Getter of the `status` attribute.
    /// [`USBInTransferResult.status`](https://developer.mozilla.org/en-US/docs/Web/API/USBInTransferResult/status)
    pub fn status(&self) -> USBTransferStatus {
        self.inner.get("status").as_::<USBTransferStatus>()
    }
}
