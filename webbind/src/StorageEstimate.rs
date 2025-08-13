use super::*;




/// The StorageEstimate dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageEstimate {
    inner: Any,
}

impl FromVal for StorageEstimate {
    fn from_val(v: &Any) -> Self {
        StorageEstimate { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StorageEstimate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StorageEstimate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StorageEstimate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StorageEstimate {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StorageEstimate> for Any {
    fn from(s: StorageEstimate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StorageEstimate> for Any {
    fn from(s: &StorageEstimate) -> Any {
        s.inner.clone()
    }
}

impl StorageEstimate {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> u64 {
        self.inner.get("usage").as_::<u64>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: u64) {
        self.inner.set("usage", value);
    }
}
impl StorageEstimate {
    /// Getter of the `quota` attribute.
    pub fn quota(&self) -> u64 {
        self.inner.get("quota").as_::<u64>()
    }

    /// Setter of the `quota` attribute.
    pub fn set_quota(&mut self, value: u64) {
        self.inner.set("quota", value);
    }
}
