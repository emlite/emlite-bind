use super::*;




/// The BackgroundSyncOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundSyncOptions {
    inner: Any,
}

impl FromVal for BackgroundSyncOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundSyncOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BackgroundSyncOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BackgroundSyncOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BackgroundSyncOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundSyncOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BackgroundSyncOptions> for Any {
    fn from(s: BackgroundSyncOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundSyncOptions> for Any {
    fn from(s: &BackgroundSyncOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundSyncOptions {
    /// Getter of the `minInterval` attribute.
    pub fn min_interval(&self) -> u64 {
        self.inner.get("minInterval").as_::<u64>()
    }

    /// Setter of the `minInterval` attribute.
    pub fn set_min_interval(&mut self, value: u64) {
        self.inner.set("minInterval", value);
    }
}
