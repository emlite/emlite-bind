use super::*;




/// The NavigationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationOptions {
    inner: Any,
}

impl FromVal for NavigationOptions {
    fn from_val(v: &Any) -> Self {
        NavigationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NavigationOptions> for Any {
    fn from(s: NavigationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationOptions> for Any {
    fn from(s: &NavigationOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationOptions {
    /// Getter of the `info` attribute.
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }

    /// Setter of the `info` attribute.
    pub fn set_info(&mut self, value: &Any) {
        self.inner.set("info", value);
    }
}
