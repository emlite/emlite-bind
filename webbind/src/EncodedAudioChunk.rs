use super::*;

/// The EncodedAudioChunk class.
/// [`EncodedAudioChunk`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedAudioChunk {
    inner: Any,
}

impl FromVal for EncodedAudioChunk {
    fn from_val(v: &Any) -> Self {
        EncodedAudioChunk {
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

impl core::ops::Deref for EncodedAudioChunk {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedAudioChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedAudioChunk {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedAudioChunk {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EncodedAudioChunk> for Any {
    fn from(s: EncodedAudioChunk) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedAudioChunk> for Any {
    fn from(s: &EncodedAudioChunk) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EncodedAudioChunk);

impl EncodedAudioChunk {
    /// The `new EncodedAudioChunk(..)` constructor, creating a new EncodedAudioChunk instance
    pub fn new(init: &EncodedAudioChunkInit) -> EncodedAudioChunk {
        Self {
            inner: Any::global("EncodedAudioChunk")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl EncodedAudioChunk {
    /// Getter of the `type` attribute.
    /// [`EncodedAudioChunk.type`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/type)
    pub fn type_(&self) -> EncodedAudioChunkType {
        self.inner.get("type").as_::<EncodedAudioChunkType>()
    }
}
impl EncodedAudioChunk {
    /// Getter of the `timestamp` attribute.
    /// [`EncodedAudioChunk.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/timestamp)
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl EncodedAudioChunk {
    /// Getter of the `duration` attribute.
    /// [`EncodedAudioChunk.duration`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/duration)
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl EncodedAudioChunk {
    /// Getter of the `byteLength` attribute.
    /// [`EncodedAudioChunk.byteLength`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/byteLength)
    pub fn byte_length(&self) -> u32 {
        self.inner.get("byteLength").as_::<u32>()
    }
}
impl EncodedAudioChunk {
    /// The copyTo method.
    /// [`EncodedAudioChunk.copyTo`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)
    pub fn copy_to(&self, destination: &Any) -> Undefined {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<Undefined>()
    }
}
