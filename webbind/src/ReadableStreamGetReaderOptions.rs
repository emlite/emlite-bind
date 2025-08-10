use super::*;

/// The ReadableStreamGetReaderOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamGetReaderOptions {
    inner: Any,
}

impl FromVal for ReadableStreamGetReaderOptions {
    fn from_val(v: &Any) -> Self {
        ReadableStreamGetReaderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableStreamGetReaderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableStreamGetReaderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableStreamGetReaderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableStreamGetReaderOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadableStreamGetReaderOptions> for Any {
    fn from(s: ReadableStreamGetReaderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableStreamGetReaderOptions> for Any {
    fn from(s: &ReadableStreamGetReaderOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadableStreamGetReaderOptions {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> ReadableStreamReaderMode {
        self.inner.get("mode").as_::<ReadableStreamReaderMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &ReadableStreamReaderMode) {
        self.inner.set("mode", value);
    }
}
