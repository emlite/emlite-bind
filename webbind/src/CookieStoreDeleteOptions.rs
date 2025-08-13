use super::*;




/// The CookieStoreDeleteOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStoreDeleteOptions {
    inner: Any,
}

impl FromVal for CookieStoreDeleteOptions {
    fn from_val(v: &Any) -> Self {
        CookieStoreDeleteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CookieStoreDeleteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CookieStoreDeleteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CookieStoreDeleteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CookieStoreDeleteOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CookieStoreDeleteOptions> for Any {
    fn from(s: CookieStoreDeleteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CookieStoreDeleteOptions> for Any {
    fn from(s: &CookieStoreDeleteOptions) -> Any {
        s.inner.clone()
    }
}

impl CookieStoreDeleteOptions {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl CookieStoreDeleteOptions {
    /// Getter of the `domain` attribute.
    pub fn domain(&self) -> JsString {
        self.inner.get("domain").as_::<JsString>()
    }

    /// Setter of the `domain` attribute.
    pub fn set_domain(&mut self, value: &JsString) {
        self.inner.set("domain", value);
    }
}
impl CookieStoreDeleteOptions {
    /// Getter of the `path` attribute.
    pub fn path(&self) -> JsString {
        self.inner.get("path").as_::<JsString>()
    }

    /// Setter of the `path` attribute.
    pub fn set_path(&mut self, value: &JsString) {
        self.inner.set("path", value);
    }
}
impl CookieStoreDeleteOptions {
    /// Getter of the `partitioned` attribute.
    pub fn partitioned(&self) -> bool {
        self.inner.get("partitioned").as_::<bool>()
    }

    /// Setter of the `partitioned` attribute.
    pub fn set_partitioned(&mut self, value: bool) {
        self.inner.set("partitioned", value);
    }
}
