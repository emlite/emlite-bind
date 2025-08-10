use super::*;

/// The CookieListItem dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieListItem {
    inner: Any,
}

impl FromVal for CookieListItem {
    fn from_val(v: &Any) -> Self {
        CookieListItem { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CookieListItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CookieListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CookieListItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CookieListItem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CookieListItem> for Any {
    fn from(s: CookieListItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CookieListItem> for Any {
    fn from(s: &CookieListItem) -> Any {
        s.inner.clone()
    }
}

impl CookieListItem {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl CookieListItem {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl CookieListItem {
    /// Getter of the `domain` attribute.
    pub fn domain(&self) -> JsString {
        self.inner.get("domain").as_::<JsString>()
    }

    /// Setter of the `domain` attribute.
    pub fn set_domain(&mut self, value: &JsString) {
        self.inner.set("domain", value);
    }
}
impl CookieListItem {
    /// Getter of the `path` attribute.
    pub fn path(&self) -> JsString {
        self.inner.get("path").as_::<JsString>()
    }

    /// Setter of the `path` attribute.
    pub fn set_path(&mut self, value: &JsString) {
        self.inner.set("path", value);
    }
}
impl CookieListItem {
    /// Getter of the `expires` attribute.
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    /// Setter of the `expires` attribute.
    pub fn set_expires(&mut self, value: &Any) {
        self.inner.set("expires", value);
    }
}
impl CookieListItem {
    /// Getter of the `secure` attribute.
    pub fn secure(&self) -> bool {
        self.inner.get("secure").as_::<bool>()
    }

    /// Setter of the `secure` attribute.
    pub fn set_secure(&mut self, value: bool) {
        self.inner.set("secure", value);
    }
}
impl CookieListItem {
    /// Getter of the `sameSite` attribute.
    pub fn same_site(&self) -> CookieSameSite {
        self.inner.get("sameSite").as_::<CookieSameSite>()
    }

    /// Setter of the `sameSite` attribute.
    pub fn set_same_site(&mut self, value: &CookieSameSite) {
        self.inner.set("sameSite", value);
    }
}
impl CookieListItem {
    /// Getter of the `partitioned` attribute.
    pub fn partitioned(&self) -> bool {
        self.inner.get("partitioned").as_::<bool>()
    }

    /// Setter of the `partitioned` attribute.
    pub fn set_partitioned(&mut self, value: bool) {
        self.inner.set("partitioned", value);
    }
}
