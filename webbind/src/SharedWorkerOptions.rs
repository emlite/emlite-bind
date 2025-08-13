use super::*;




/// The SharedWorkerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedWorkerOptions {
    inner: Any,
}

impl FromVal for SharedWorkerOptions {
    fn from_val(v: &Any) -> Self {
        SharedWorkerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedWorkerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedWorkerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedWorkerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedWorkerOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SharedWorkerOptions> for Any {
    fn from(s: SharedWorkerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedWorkerOptions> for Any {
    fn from(s: &SharedWorkerOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedWorkerOptions {
    /// Getter of the `sameSiteCookies` attribute.
    pub fn same_site_cookies(&self) -> SameSiteCookiesType {
        self.inner.get("sameSiteCookies").as_::<SameSiteCookiesType>()
    }

    /// Setter of the `sameSiteCookies` attribute.
    pub fn set_same_site_cookies(&mut self, value: &SameSiteCookiesType) {
        self.inner.set("sameSiteCookies", value);
    }
}
