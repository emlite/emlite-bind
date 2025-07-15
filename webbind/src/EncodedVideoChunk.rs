use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for EncodedVideoChunk {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EncodedVideoChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for EncodedVideoChunk {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for EncodedVideoChunk {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<EncodedVideoChunk> for emlite::Val {
    fn from(s: EncodedVideoChunk) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&EncodedVideoChunk> for emlite::Val {
    fn from(s: &EncodedVideoChunk) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EncodedVideoChunk);

impl EncodedVideoChunk {
    pub fn new(init: &Any) -> EncodedVideoChunk {
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
    pub fn copy_to(&self, destination: &Any) -> Undefined {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<Undefined>()
    }
}
