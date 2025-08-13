use super::*;




/// The EncodedVideoChunkInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedVideoChunkInit {
    inner: Any,
}

impl FromVal for EncodedVideoChunkInit {
    fn from_val(v: &Any) -> Self {
        EncodedVideoChunkInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncodedVideoChunkInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedVideoChunkInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedVideoChunkInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedVideoChunkInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EncodedVideoChunkInit> for Any {
    fn from(s: EncodedVideoChunkInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedVideoChunkInit> for Any {
    fn from(s: &EncodedVideoChunkInit) -> Any {
        s.inner.clone()
    }
}

impl EncodedVideoChunkInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> EncodedVideoChunkType {
        self.inner.get("type").as_::<EncodedVideoChunkType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &EncodedVideoChunkType) {
        self.inner.set("type", value);
    }
}
impl EncodedVideoChunkInit {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl EncodedVideoChunkInit {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: u64) {
        self.inner.set("duration", value);
    }
}
impl EncodedVideoChunkInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl EncodedVideoChunkInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}
