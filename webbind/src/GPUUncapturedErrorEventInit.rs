use super::*;

/// The GPUUncapturedErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUUncapturedErrorEventInit {
    inner: Any,
}

impl FromVal for GPUUncapturedErrorEventInit {
    fn from_val(v: &Any) -> Self {
        GPUUncapturedErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUUncapturedErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUUncapturedErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUUncapturedErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUUncapturedErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUUncapturedErrorEventInit> for Any {
    fn from(s: GPUUncapturedErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUUncapturedErrorEventInit> for Any {
    fn from(s: &GPUUncapturedErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl GPUUncapturedErrorEventInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> GPUError {
        self.inner.get("error").as_::<GPUError>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &GPUError) {
        self.inner.set("error", value);
    }
}
