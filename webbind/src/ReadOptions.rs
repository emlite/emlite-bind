use super::*;

/// The ReadOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadOptions {
    inner: Any,
}

impl FromVal for ReadOptions {
    fn from_val(v: &Any) -> Self {
        ReadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadOptions> for Any {
    fn from(s: ReadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadOptions> for Any {
    fn from(s: &ReadOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
