use super::*;

/// The GPUCompilationInfo class.
/// [`GPUCompilationInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCompilationInfo {
    inner: Any,
}

impl FromVal for GPUCompilationInfo {
    fn from_val(v: &Any) -> Self {
        GPUCompilationInfo {
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

impl core::ops::Deref for GPUCompilationInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUCompilationInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUCompilationInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUCompilationInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUCompilationInfo> for Any {
    fn from(s: GPUCompilationInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUCompilationInfo> for Any {
    fn from(s: &GPUCompilationInfo) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUCompilationInfo);

impl GPUCompilationInfo {
    /// Getter of the `messages` attribute.
    /// [`GPUCompilationInfo.messages`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationInfo/messages)
    pub fn messages(&self) -> TypedArray<GPUCompilationMessage> {
        self.inner
            .get("messages")
            .as_::<TypedArray<GPUCompilationMessage>>()
    }
}
