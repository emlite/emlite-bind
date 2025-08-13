use super::*;




/// The ReadableStreamBYOBReaderReadOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamBYOBReaderReadOptions {
    inner: Any,
}

impl FromVal for ReadableStreamBYOBReaderReadOptions {
    fn from_val(v: &Any) -> Self {
        ReadableStreamBYOBReaderReadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableStreamBYOBReaderReadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableStreamBYOBReaderReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableStreamBYOBReaderReadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableStreamBYOBReaderReadOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ReadableStreamBYOBReaderReadOptions> for Any {
    fn from(s: ReadableStreamBYOBReaderReadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableStreamBYOBReaderReadOptions> for Any {
    fn from(s: &ReadableStreamBYOBReaderReadOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadableStreamBYOBReaderReadOptions {
    /// Getter of the `min` attribute.
    pub fn min(&self) -> u64 {
        self.inner.get("min").as_::<u64>()
    }

    /// Setter of the `min` attribute.
    pub fn set_min(&mut self, value: u64) {
        self.inner.set("min", value);
    }
}
