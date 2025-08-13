use super::*;




/// The StorageBucketOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucketOptions {
    inner: Any,
}

impl FromVal for StorageBucketOptions {
    fn from_val(v: &Any) -> Self {
        StorageBucketOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StorageBucketOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StorageBucketOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StorageBucketOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StorageBucketOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StorageBucketOptions> for Any {
    fn from(s: StorageBucketOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StorageBucketOptions> for Any {
    fn from(s: &StorageBucketOptions) -> Any {
        s.inner.clone()
    }
}

impl StorageBucketOptions {
    /// Getter of the `persisted` attribute.
    pub fn persisted(&self) -> bool {
        self.inner.get("persisted").as_::<bool>()
    }

    /// Setter of the `persisted` attribute.
    pub fn set_persisted(&mut self, value: bool) {
        self.inner.set("persisted", value);
    }
}
impl StorageBucketOptions {
    /// Getter of the `quota` attribute.
    pub fn quota(&self) -> u64 {
        self.inner.get("quota").as_::<u64>()
    }

    /// Setter of the `quota` attribute.
    pub fn set_quota(&mut self, value: u64) {
        self.inner.set("quota", value);
    }
}
impl StorageBucketOptions {
    /// Getter of the `expires` attribute.
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    /// Setter of the `expires` attribute.
    pub fn set_expires(&mut self, value: &Any) {
        self.inner.set("expires", value);
    }
}
