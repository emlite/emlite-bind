use super::*;

/// The EncodedVideoChunk class.
/// [`EncodedVideoChunk`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedVideoChunk {
    inner: Any,
}

impl FromVal for EncodedVideoChunk {
    fn from_val(v: &Any) -> Self {
        EncodedVideoChunk {
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

impl core::ops::Deref for EncodedVideoChunk {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedVideoChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedVideoChunk {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedVideoChunk {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EncodedVideoChunk> for Any {
    fn from(s: EncodedVideoChunk) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedVideoChunk> for Any {
    fn from(s: &EncodedVideoChunk) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EncodedVideoChunk);

impl EncodedVideoChunk {
    /// Getter of the `type` attribute.
    /// [`EncodedVideoChunk.type`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/type)
    pub fn type_(&self) -> EncodedVideoChunkType {
        self.inner.get("type").as_::<EncodedVideoChunkType>()
    }
}
impl EncodedVideoChunk {
    /// Getter of the `timestamp` attribute.
    /// [`EncodedVideoChunk.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/timestamp)
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl EncodedVideoChunk {
    /// Getter of the `duration` attribute.
    /// [`EncodedVideoChunk.duration`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/duration)
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl EncodedVideoChunk {
    /// Getter of the `byteLength` attribute.
    /// [`EncodedVideoChunk.byteLength`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/byteLength)
    pub fn byte_length(&self) -> u32 {
        self.inner.get("byteLength").as_::<u32>()
    }
}

impl EncodedVideoChunk {
    /// The `new EncodedVideoChunk(..)` constructor, creating a new EncodedVideoChunk instance
    pub fn new(init: &EncodedVideoChunkInit) -> EncodedVideoChunk {
        Self {
            inner: Any::global("EncodedVideoChunk")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}

impl EncodedVideoChunk {
    /// The copyTo method.
    /// [`EncodedVideoChunk.copyTo`](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)
    pub fn copy_to(&self, destination: &Any) -> Undefined {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<Undefined>()
    }
}
