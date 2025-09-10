use super::*;

/// The NDEFScanOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFScanOptions {
    inner: Any,
}

impl FromVal for NDEFScanOptions {
    fn from_val(v: &Any) -> Self {
        NDEFScanOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFScanOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFScanOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFScanOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NDEFScanOptions> for Any {
    fn from(s: NDEFScanOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFScanOptions> for Any {
    fn from(s: &NDEFScanOptions) -> Any {
        s.inner.clone()
    }
}

impl NDEFScanOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
