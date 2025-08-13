use super::*;




/// The CacheQueryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CacheQueryOptions {
    inner: Any,
}

impl FromVal for CacheQueryOptions {
    fn from_val(v: &Any) -> Self {
        CacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CacheQueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CacheQueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CacheQueryOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CacheQueryOptions> for Any {
    fn from(s: CacheQueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CacheQueryOptions> for Any {
    fn from(s: &CacheQueryOptions) -> Any {
        s.inner.clone()
    }
}

impl CacheQueryOptions {
    /// Getter of the `ignoreSearch` attribute.
    pub fn ignore_search(&self) -> bool {
        self.inner.get("ignoreSearch").as_::<bool>()
    }

    /// Setter of the `ignoreSearch` attribute.
    pub fn set_ignore_search(&mut self, value: bool) {
        self.inner.set("ignoreSearch", value);
    }
}
impl CacheQueryOptions {
    /// Getter of the `ignoreMethod` attribute.
    pub fn ignore_method(&self) -> bool {
        self.inner.get("ignoreMethod").as_::<bool>()
    }

    /// Setter of the `ignoreMethod` attribute.
    pub fn set_ignore_method(&mut self, value: bool) {
        self.inner.set("ignoreMethod", value);
    }
}
impl CacheQueryOptions {
    /// Getter of the `ignoreVary` attribute.
    pub fn ignore_vary(&self) -> bool {
        self.inner.get("ignoreVary").as_::<bool>()
    }

    /// Setter of the `ignoreVary` attribute.
    pub fn set_ignore_vary(&mut self, value: bool) {
        self.inner.set("ignoreVary", value);
    }
}
