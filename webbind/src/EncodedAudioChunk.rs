use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedAudioChunk {
    inner: emlite::Val,
}
impl FromVal for EncodedAudioChunk {
    fn from_val(v: &emlite::Val) -> Self {
        EncodedAudioChunk {
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
impl core::ops::Deref for EncodedAudioChunk {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EncodedAudioChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for EncodedAudioChunk {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for EncodedAudioChunk {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<EncodedAudioChunk> for emlite::Val {
    fn from(s: EncodedAudioChunk) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(EncodedAudioChunk);

impl EncodedAudioChunk {
    pub fn new(init: Any) -> EncodedAudioChunk {
        Self {
            inner: emlite::Val::global("EncodedAudioChunk")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl EncodedAudioChunk {
    pub fn type_(&self) -> EncodedAudioChunkType {
        self.inner.get("type").as_::<EncodedAudioChunkType>()
    }
}
impl EncodedAudioChunk {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl EncodedAudioChunk {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl EncodedAudioChunk {
    pub fn byte_length(&self) -> u32 {
        self.inner.get("byteLength").as_::<u32>()
    }
}
impl EncodedAudioChunk {
    pub fn copy_to(&self, destination: Any) -> Undefined {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<Undefined>()
    }
}
