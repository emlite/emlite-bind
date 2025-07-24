use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecodeOptions {
    inner: Any,
}
impl FromVal for TextDecodeOptions {
    fn from_val(v: &Any) -> Self {
        TextDecodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextDecodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextDecodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextDecodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextDecodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextDecodeOptions> for Any {
    fn from(s: TextDecodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextDecodeOptions> for Any {
    fn from(s: &TextDecodeOptions) -> Any {
        s.inner.clone()
    }
}

impl TextDecodeOptions {
    pub fn stream(&self) -> bool {
        self.inner.get("stream").as_::<bool>()
    }

    pub fn set_stream(&mut self, value: bool) {
        self.inner.set("stream", value);
    }
}
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
    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new0() -> TextDecoder {
        Self {
            inner: Any::global("TextDecoder").new(&[]).as_::<Any>(),
        }
    }

    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new1(label: &DOMString) -> TextDecoder {
        Self {
            inner: Any::global("TextDecoder").new(&[label.into()]).as_::<Any>(),
        }
    }

    /// The `new TextDecoder(..)` constructor, creating a new TextDecoder instance
    pub fn new2(label: &DOMString, options: &Any) -> TextDecoder {
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
    pub fn decode0(&self) -> USVString {
        self.inner.call("decode", &[]).as_::<USVString>()
    }
    /// The decode method.
    /// [`TextDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    pub fn decode1(&self, input: &Any) -> USVString {
        self.inner
            .call("decode", &[input.into()])
            .as_::<USVString>()
    }
    /// The decode method.
    /// [`TextDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    pub fn decode2(&self, input: &Any, options: &TextDecodeOptions) -> USVString {
        self.inner
            .call("decode", &[input.into(), options.into()])
            .as_::<USVString>()
    }
}
impl TextDecoder {
    /// Getter of the `encoding` attribute.
    /// [`TextDecoder.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)
    pub fn encoding(&self) -> DOMString {
        self.inner.get("encoding").as_::<DOMString>()
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
