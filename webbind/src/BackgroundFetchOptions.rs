use super::*;




/// The BackgroundFetchOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchOptions {
    inner: Any,
}

impl FromVal for BackgroundFetchOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BackgroundFetchOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BackgroundFetchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BackgroundFetchOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundFetchOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BackgroundFetchOptions> for Any {
    fn from(s: BackgroundFetchOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundFetchOptions> for Any {
    fn from(s: &BackgroundFetchOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundFetchOptions {
    /// Getter of the `downloadTotal` attribute.
    pub fn download_total(&self) -> u64 {
        self.inner.get("downloadTotal").as_::<u64>()
    }

    /// Setter of the `downloadTotal` attribute.
    pub fn set_download_total(&mut self, value: u64) {
        self.inner.set("downloadTotal", value);
    }
}
