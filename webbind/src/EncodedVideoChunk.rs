use super::*;

#[derive(Clone, Debug)]
pub struct EncodedVideoChunk {
    inner: emlite::Val,
}
impl FromVal for EncodedVideoChunk {
    fn from_val(v: &emlite::Val) -> Self {
        EncodedVideoChunk {
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
impl std::ops::Deref for EncodedVideoChunk {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EncodedVideoChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EncodedVideoChunk> for emlite::Val {
    fn from(s: EncodedVideoChunk) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EncodedVideoChunk {
    pub fn new(init: jsbind::Any) -> EncodedVideoChunk {
        Self {
            inner: emlite::Val::global("EncodedVideoChunk")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl EncodedVideoChunk {
    pub fn type_(&self) -> EncodedVideoChunkType {
        self.inner.get("type").as_::<EncodedVideoChunkType>()
    }
}
impl EncodedVideoChunk {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl EncodedVideoChunk {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl EncodedVideoChunk {
    pub fn byte_length(&self) -> u32 {
        self.inner.get("byteLength").as_::<u32>()
    }
}
impl EncodedVideoChunk {
    pub fn copy_to(&self, destination: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<jsbind::Undefined>()
    }
}
