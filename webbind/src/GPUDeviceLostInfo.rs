use super::*;

/// The GPUDeviceLostInfo class.
/// [`GPUDeviceLostInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDeviceLostInfo {
    inner: Any,
}
impl FromVal for GPUDeviceLostInfo {
    fn from_val(v: &Any) -> Self {
        GPUDeviceLostInfo {
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
impl core::ops::Deref for GPUDeviceLostInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUDeviceLostInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUDeviceLostInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUDeviceLostInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUDeviceLostInfo> for Any {
    fn from(s: GPUDeviceLostInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUDeviceLostInfo> for Any {
    fn from(s: &GPUDeviceLostInfo) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUDeviceLostInfo);

impl GPUDeviceLostInfo {
    /// Getter of the `reason` attribute.
    /// [`GPUDeviceLostInfo.reason`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo/reason)
    pub fn reason(&self) -> GPUDeviceLostReason {
        self.inner.get("reason").as_::<GPUDeviceLostReason>()
    }
}
impl GPUDeviceLostInfo {
    /// Getter of the `message` attribute.
    /// [`GPUDeviceLostInfo.message`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo/message)
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}
