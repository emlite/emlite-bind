use super::*;




/// The MultiCacheQueryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MultiCacheQueryOptions {
    inner: Any,
}

impl FromVal for MultiCacheQueryOptions {
    fn from_val(v: &Any) -> Self {
        MultiCacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MultiCacheQueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MultiCacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MultiCacheQueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MultiCacheQueryOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MultiCacheQueryOptions> for Any {
    fn from(s: MultiCacheQueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MultiCacheQueryOptions> for Any {
    fn from(s: &MultiCacheQueryOptions) -> Any {
        s.inner.clone()
    }
}

impl MultiCacheQueryOptions {
    /// Getter of the `cacheName` attribute.
    pub fn cache_name(&self) -> JsString {
        self.inner.get("cacheName").as_::<JsString>()
    }

    /// Setter of the `cacheName` attribute.
    pub fn set_cache_name(&mut self, value: &JsString) {
        self.inner.set("cacheName", value);
    }
}
