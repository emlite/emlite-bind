use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextEncoderEncodeIntoResult {
    inner: Any,
}
impl FromVal for TextEncoderEncodeIntoResult {
    fn from_val(v: &Any) -> Self {
        TextEncoderEncodeIntoResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextEncoderEncodeIntoResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoderEncodeIntoResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextEncoderEncodeIntoResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextEncoderEncodeIntoResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextEncoderEncodeIntoResult> for Any {
    fn from(s: TextEncoderEncodeIntoResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextEncoderEncodeIntoResult> for Any {
    fn from(s: &TextEncoderEncodeIntoResult) -> Any {
        s.inner.clone()
    }
}

impl TextEncoderEncodeIntoResult {
    pub fn read(&self) -> u64 {
        self.inner.get("read").as_::<u64>()
    }

    pub fn set_read(&mut self, value: u64) {
        self.inner.set("read", value);
    }
}
impl TextEncoderEncodeIntoResult {
    pub fn written(&self) -> u64 {
        self.inner.get("written").as_::<u64>()
    }

    pub fn set_written(&mut self, value: u64) {
        self.inner.set("written", value);
    }
}
/// The TextEncoder class.
/// [`TextEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextEncoder {
    inner: Any,
}
impl FromVal for TextEncoder {
    fn from_val(v: &Any) -> Self {
        TextEncoder {
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
impl core::ops::Deref for TextEncoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextEncoder> for Any {
    fn from(s: TextEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextEncoder> for Any {
    fn from(s: &TextEncoder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextEncoder);

impl TextEncoder {
    /// The `new TextEncoder(..)` constructor, creating a new TextEncoder instance
    pub fn new() -> TextEncoder {
        Self {
            inner: Any::global("TextEncoder").new(&[]).as_::<Any>(),
        }
    }
}
impl TextEncoder {
    /// The encode method.
    /// [`TextEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)
    pub fn encode0(&self) -> Uint8Array {
        self.inner.call("encode", &[]).as_::<Uint8Array>()
    }
    /// The encode method.
    /// [`TextEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)
    pub fn encode1(&self, input: &JsString) -> Uint8Array {
        self.inner
            .call("encode", &[input.into()])
            .as_::<Uint8Array>()
    }
}
impl TextEncoder {
    /// The encodeInto method.
    /// [`TextEncoder.encodeInto`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encodeInto)
    pub fn encode_into(
        &self,
        source: &JsString,
        destination: &Uint8Array,
    ) -> TextEncoderEncodeIntoResult {
        self.inner
            .call("encodeInto", &[source.into(), destination.into()])
            .as_::<TextEncoderEncodeIntoResult>()
    }
}
impl TextEncoder {
    /// Getter of the `encoding` attribute.
    /// [`TextEncoder.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }
}
