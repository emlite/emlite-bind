use super::*;

#[derive(Clone, Debug)]
pub struct TextDecoderStream {
    inner: emlite::Val,
}
impl FromVal for TextDecoderStream {
    fn from_val(v: &emlite::Val) -> Self {
        TextDecoderStream {
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
impl std::ops::Deref for TextDecoderStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TextDecoderStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextDecoderStream> for emlite::Val {
    fn from(s: TextDecoderStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextDecoderStream {
    pub fn new0() -> TextDecoderStream {
        Self {
            inner: emlite::Val::global("TextDecoderStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(label: jsbind::DOMString) -> TextDecoderStream {
        Self {
            inner: emlite::Val::global("TextDecoderStream")
                .new(&[label.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(label: jsbind::DOMString, options: jsbind::Any) -> TextDecoderStream {
        Self {
            inner: emlite::Val::global("TextDecoderStream")
                .new(&[label.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextDecoderStream {
    pub fn encoding(&self) -> jsbind::DOMString {
        self.inner.get("encoding").as_::<jsbind::DOMString>()
    }
}
impl TextDecoderStream {
    pub fn fatal(&self) -> bool {
        self.inner.get("fatal").as_::<bool>()
    }
}
impl TextDecoderStream {
    pub fn ignore_bom(&self) -> bool {
        self.inner.get("ignoreBOM").as_::<bool>()
    }
}
impl TextDecoderStream {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl TextDecoderStream {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
