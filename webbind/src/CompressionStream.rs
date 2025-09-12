use super::*;

/// The CompressionStream class.
/// [`CompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CompressionStream {
    inner: Any,
}

impl FromVal for CompressionStream {
    fn from_val(v: &Any) -> Self {
        CompressionStream {
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

impl core::ops::Deref for CompressionStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CompressionStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CompressionStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CompressionStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CompressionStream> for Any {
    fn from(s: CompressionStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CompressionStream> for Any {
    fn from(s: &CompressionStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CompressionStream);

impl CompressionStream {
    /// Getter of the `readable` attribute.
    /// [`CompressionStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl CompressionStream {
    /// Getter of the `writable` attribute.
    /// [`CompressionStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}

impl CompressionStream {
    /// The `new CompressionStream(..)` constructor, creating a new CompressionStream instance
    pub fn new(format: &CompressionFormat) -> CompressionStream {
        Self {
            inner: Any::global("CompressionStream")
                .new(&[format.into()])
                .as_::<Any>(),
        }
    }
}
