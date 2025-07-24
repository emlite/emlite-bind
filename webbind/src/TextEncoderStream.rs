use super::*;

/// The TextEncoderStream class.
/// [`TextEncoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextEncoderStream {
    inner: Any,
}
impl FromVal for TextEncoderStream {
    fn from_val(v: &Any) -> Self {
        TextEncoderStream {
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
impl core::ops::Deref for TextEncoderStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoderStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextEncoderStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextEncoderStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextEncoderStream> for Any {
    fn from(s: TextEncoderStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextEncoderStream> for Any {
    fn from(s: &TextEncoderStream) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextEncoderStream);

impl TextEncoderStream {
    /// The `new TextEncoderStream(..)` constructor, creating a new TextEncoderStream instance
    pub fn new() -> TextEncoderStream {
        Self {
            inner: Any::global("TextEncoderStream").new(&[]).as_::<Any>(),
        }
    }
}
impl TextEncoderStream {
    /// Getter of the `encoding` attribute.
    /// [`TextEncoderStream.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream/encoding)
    pub fn encoding(&self) -> DOMString {
        self.inner.get("encoding").as_::<DOMString>()
    }
}
impl TextEncoderStream {
    /// Getter of the `readable` attribute.
    /// [`TextEncoderStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl TextEncoderStream {
    /// Getter of the `writable` attribute.
    /// [`TextEncoderStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
