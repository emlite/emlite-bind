use super::*;

/// The NDEFWriteOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFWriteOptions {
    inner: Any,
}

impl FromVal for NDEFWriteOptions {
    fn from_val(v: &Any) -> Self {
        NDEFWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFWriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFWriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFWriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NDEFWriteOptions> for Any {
    fn from(s: NDEFWriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFWriteOptions> for Any {
    fn from(s: &NDEFWriteOptions) -> Any {
        s.inner.clone()
    }
}

impl NDEFWriteOptions {
    /// Getter of the `overwrite` attribute.
    pub fn overwrite(&self) -> bool {
        self.inner.get("overwrite").as_::<bool>()
    }

    /// Setter of the `overwrite` attribute.
    pub fn set_overwrite(&mut self, value: bool) {
        self.inner.set("overwrite", value);
    }
}
impl NDEFWriteOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
