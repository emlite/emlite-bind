use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecodeOptions {
    inner: emlite::Val,
}
impl FromVal for TextDecodeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        TextDecodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextDecodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextDecodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextDecodeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextDecodeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextDecodeOptions> for emlite::Val {
    fn from(s: TextDecodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TextDecodeOptions> for emlite::Val {
    fn from(s: &TextDecodeOptions) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecoder {
    inner: emlite::Val,
}
impl FromVal for TextDecoder {
    fn from_val(v: &emlite::Val) -> Self {
        TextDecoder {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextDecoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextDecoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextDecoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextDecoder> for emlite::Val {
    fn from(s: TextDecoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TextDecoder> for emlite::Val {
    fn from(s: &TextDecoder) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextDecoder);

impl TextDecoder {
    pub fn new0() -> TextDecoder {
        Self {
            inner: emlite::Val::global("TextDecoder")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(label: DOMString) -> TextDecoder {
        Self {
            inner: emlite::Val::global("TextDecoder")
                .new(&[label.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(label: DOMString, options: Any) -> TextDecoder {
        Self {
            inner: emlite::Val::global("TextDecoder")
                .new(&[label.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextDecoder {
    pub fn decode0(&self) -> USVString {
        self.inner.call("decode", &[]).as_::<USVString>()
    }

    pub fn decode1(&self, input: Any) -> USVString {
        self.inner
            .call("decode", &[input.into()])
            .as_::<USVString>()
    }

    pub fn decode2(&self, input: Any, options: TextDecodeOptions) -> USVString {
        self.inner
            .call("decode", &[input.into(), options.into()])
            .as_::<USVString>()
    }
}
impl TextDecoder {
    pub fn encoding(&self) -> DOMString {
        self.inner.get("encoding").as_::<DOMString>()
    }
}
impl TextDecoder {
    pub fn fatal(&self) -> bool {
        self.inner.get("fatal").as_::<bool>()
    }
}
impl TextDecoder {
    pub fn ignore_bom(&self) -> bool {
        self.inner.get("ignoreBOM").as_::<bool>()
    }
}
