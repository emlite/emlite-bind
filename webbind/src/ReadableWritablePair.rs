use super::*;

/// The ReadableWritablePair dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableWritablePair {
    inner: Any,
}

impl FromVal for ReadableWritablePair {
    fn from_val(v: &Any) -> Self {
        ReadableWritablePair { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableWritablePair {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableWritablePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableWritablePair {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableWritablePair {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadableWritablePair> for Any {
    fn from(s: ReadableWritablePair) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableWritablePair> for Any {
    fn from(s: &ReadableWritablePair) -> Any {
        s.inner.clone()
    }
}

impl ReadableWritablePair {
    /// Getter of the `readable` attribute.
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

    /// Setter of the `readable` attribute.
    pub fn set_readable(&mut self, value: &ReadableStream) {
        self.inner.set("readable", value);
    }
}
impl ReadableWritablePair {
    /// Getter of the `writable` attribute.
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

    /// Setter of the `writable` attribute.
    pub fn set_writable(&mut self, value: &WritableStream) {
        self.inner.set("writable", value);
    }
}
