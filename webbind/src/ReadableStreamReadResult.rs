use super::*;

/// The ReadableStreamReadResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamReadResult {
    inner: Any,
}

impl FromVal for ReadableStreamReadResult {
    fn from_val(v: &Any) -> Self {
        ReadableStreamReadResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableStreamReadResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableStreamReadResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableStreamReadResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableStreamReadResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadableStreamReadResult> for Any {
    fn from(s: ReadableStreamReadResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableStreamReadResult> for Any {
    fn from(s: &ReadableStreamReadResult) -> Any {
        s.inner.clone()
    }
}

impl ReadableStreamReadResult {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
impl ReadableStreamReadResult {
    /// Getter of the `done` attribute.
    pub fn done(&self) -> bool {
        self.inner.get("done").as_::<bool>()
    }

    /// Setter of the `done` attribute.
    pub fn set_done(&mut self, value: bool) {
        self.inner.set("done", value);
    }
}
