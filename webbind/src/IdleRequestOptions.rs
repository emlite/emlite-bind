use super::*;




/// The IdleRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleRequestOptions {
    inner: Any,
}

impl FromVal for IdleRequestOptions {
    fn from_val(v: &Any) -> Self {
        IdleRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdleRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdleRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdleRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdleRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdleRequestOptions> for Any {
    fn from(s: IdleRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdleRequestOptions> for Any {
    fn from(s: &IdleRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl IdleRequestOptions {
    /// Getter of the `timeout` attribute.
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
