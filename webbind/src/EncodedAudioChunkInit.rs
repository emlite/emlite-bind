use super::*;




/// The EncodedAudioChunkInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedAudioChunkInit {
    inner: Any,
}

impl FromVal for EncodedAudioChunkInit {
    fn from_val(v: &Any) -> Self {
        EncodedAudioChunkInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncodedAudioChunkInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedAudioChunkInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedAudioChunkInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedAudioChunkInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EncodedAudioChunkInit> for Any {
    fn from(s: EncodedAudioChunkInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedAudioChunkInit> for Any {
    fn from(s: &EncodedAudioChunkInit) -> Any {
        s.inner.clone()
    }
}

impl EncodedAudioChunkInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> EncodedAudioChunkType {
        self.inner.get("type").as_::<EncodedAudioChunkType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &EncodedAudioChunkType) {
        self.inner.set("type", value);
    }
}
impl EncodedAudioChunkInit {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl EncodedAudioChunkInit {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl EncodedAudioChunkInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl EncodedAudioChunkInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}
