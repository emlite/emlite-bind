use super::*;

/// The TextDecoder class.
/// [`TextDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecoder {
    inner: Any,
}

impl FromVal for TextDecoder {
    fn from_val(v: &Any) -> Self {
        TextDecoder {
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

impl core::ops::Deref for TextDecoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextDecoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextDecoder> for Any {
    fn from(s: TextDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextDecoder> for Any {
    fn from(s: &TextDecoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TextDecoder);

impl TextDecoder {
    /// Getter of the `encoding` attribute.
    /// [`TextDecoder.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }
}
impl TextDecoder {
    /// Getter of the `fatal` attribute.
    /// [`TextDecoder.fatal`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/fatal)
    pub fn fatal(&self) -> bool {
        self.inner.get("fatal").as_::<bool>()
    }
}
impl TextDecoder {
    /// Getter of the `ignoreBOM` attribute.
    /// [`TextDecoder.ignoreBOM`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/ignoreBOM)
    pub fn ignore_bom(&self) -> bool {
        self.inner.get("ignoreBOM").as_::<bool>()
    }
}

impl TextDecoder {
    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new() -> TextDecoder {
        Self {
            inner: Any::global("TextDecoder").new(&[]).as_::<Any>(),
        }
    }
}

impl TextDecoder {
    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new_with_label(label: &JsString) -> TextDecoder {
        Self {
            inner: Any::global("TextDecoder").new(&[label.into()]).as_::<Any>(),
        }
    }
}

impl TextDecoder {
    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new_with_label_and_options(
        label: &JsString,
        options: &TextDecoderOptions,
    ) -> TextDecoder {
        Self {
            inner: Any::global("TextDecoder")
                .new(&[label.into(), options.into()])
                .as_::<Any>(),
        }
    }
}

impl TextDecoder {
    /// The decode method.
    /// [`TextDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    pub fn decode(&self) -> JsString {
        self.inner.call("decode", &[]).as_::<JsString>()
    }
}
impl TextDecoder {
    /// The decode method.
    /// [`TextDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    pub fn decode_with_input(&self, input: &Any) -> JsString {
        self.inner.call("decode", &[input.into()]).as_::<JsString>()
    }
}
impl TextDecoder {
    /// The decode method.
    /// [`TextDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    pub fn decode_with_input_and_options(
        &self,
        input: &Any,
        options: &TextDecodeOptions,
    ) -> JsString {
        self.inner
            .call("decode", &[input.into(), options.into()])
            .as_::<JsString>()
    }
}
