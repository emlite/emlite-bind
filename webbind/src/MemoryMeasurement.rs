use super::*;




/// The MemoryMeasurement dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryMeasurement {
    inner: Any,
}

impl FromVal for MemoryMeasurement {
    fn from_val(v: &Any) -> Self {
        MemoryMeasurement { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MemoryMeasurement {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MemoryMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MemoryMeasurement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MemoryMeasurement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MemoryMeasurement> for Any {
    fn from(s: MemoryMeasurement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MemoryMeasurement> for Any {
    fn from(s: &MemoryMeasurement) -> Any {
        s.inner.clone()
    }
}

impl MemoryMeasurement {
    /// Getter of the `bytes` attribute.
    pub fn bytes(&self) -> u64 {
        self.inner.get("bytes").as_::<u64>()
    }

    /// Setter of the `bytes` attribute.
    pub fn set_bytes(&mut self, value: u64) {
        self.inner.set("bytes", value);
    }
}
impl MemoryMeasurement {
    /// Getter of the `breakdown` attribute.
    pub fn breakdown(&self) -> TypedArray<MemoryBreakdownEntry> {
        self.inner.get("breakdown").as_::<TypedArray<MemoryBreakdownEntry>>()
    }

    /// Setter of the `breakdown` attribute.
    pub fn set_breakdown(&mut self, value: &TypedArray<MemoryBreakdownEntry>) {
        self.inner.set("breakdown", value);
    }
}
