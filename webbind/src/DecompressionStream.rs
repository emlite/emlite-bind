use super::*;




/// The DecompressionStream class.
/// [`DecompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DecompressionStream {
    inner: Any,
}

impl FromVal for DecompressionStream {
    fn from_val(v: &Any) -> Self {
        DecompressionStream { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DecompressionStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DecompressionStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DecompressionStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DecompressionStream {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DecompressionStream> for Any {
    fn from(s: DecompressionStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DecompressionStream> for Any {
    fn from(s: &DecompressionStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DecompressionStream);



impl DecompressionStream {
    /// The `new DecompressionStream(..)` constructor, creating a new DecompressionStream instance
    pub fn new(format: &CompressionFormat) -> DecompressionStream {
        Self {
            inner: Any::global("DecompressionStream").new(&[format.into()]).as_::<Any>(),
        }
    }

}
impl DecompressionStream {
    /// Getter of the `readable` attribute.
    /// [`DecompressionStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

}
impl DecompressionStream {
    /// Getter of the `writable` attribute.
    /// [`DecompressionStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

}
