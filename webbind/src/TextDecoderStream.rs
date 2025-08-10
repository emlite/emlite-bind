use super::*;

/// The TextDecoderStream class.
/// [`TextDecoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecoderStream {
    inner: Any,
}

impl FromVal for TextDecoderStream {
    fn from_val(v: &Any) -> Self {
        TextDecoderStream {
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

impl core::ops::Deref for TextDecoderStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextDecoderStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextDecoderStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextDecoderStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextDecoderStream> for Any {
    fn from(s: TextDecoderStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextDecoderStream> for Any {
    fn from(s: &TextDecoderStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TextDecoderStream);

impl TextDecoderStream {
    /// The `new TextDecoderStream(..)` constructor, creating a new TextDecoderStream instance
    pub fn new0() -> TextDecoderStream {
        Self {
            inner: Any::global("TextDecoderStream").new(&[]).as_::<Any>(),
        }
    }

    /// The `new TextDecoderStream(..)` constructor, creating a new TextDecoderStream instance
    pub fn new1(label: &JsString) -> TextDecoderStream {
        Self {
            inner: Any::global("TextDecoderStream")
                .new(&[label.into()])
                .as_::<Any>(),
        }
    }

    /// The `new TextDecoderStream(..)` constructor, creating a new TextDecoderStream instance
    pub fn new2(label: &JsString, options: &TextDecoderOptions) -> TextDecoderStream {
        Self {
            inner: Any::global("TextDecoderStream")
                .new(&[label.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl TextDecoderStream {
    /// Getter of the `encoding` attribute.
    /// [`TextDecoderStream.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream/encoding)
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }
}
impl TextDecoderStream {
    /// Getter of the `fatal` attribute.
    /// [`TextDecoderStream.fatal`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream/fatal)
    pub fn fatal(&self) -> bool {
        self.inner.get("fatal").as_::<bool>()
    }
}
impl TextDecoderStream {
    /// Getter of the `ignoreBOM` attribute.
    /// [`TextDecoderStream.ignoreBOM`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream/ignoreBOM)
    pub fn ignore_bom(&self) -> bool {
        self.inner.get("ignoreBOM").as_::<bool>()
    }
}
impl TextDecoderStream {
    /// Getter of the `readable` attribute.
    /// [`TextDecoderStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl TextDecoderStream {
    /// Getter of the `writable` attribute.
    /// [`TextDecoderStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
