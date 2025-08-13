use super::*;




/// The IdleOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleOptions {
    inner: Any,
}

impl FromVal for IdleOptions {
    fn from_val(v: &Any) -> Self {
        IdleOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdleOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdleOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdleOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdleOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdleOptions> for Any {
    fn from(s: IdleOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdleOptions> for Any {
    fn from(s: &IdleOptions) -> Any {
        s.inner.clone()
    }
}

impl IdleOptions {
    /// Getter of the `threshold` attribute.
    pub fn threshold(&self) -> u64 {
        self.inner.get("threshold").as_::<u64>()
    }

    /// Setter of the `threshold` attribute.
    pub fn set_threshold(&mut self, value: u64) {
        self.inner.set("threshold", value);
    }
}
impl IdleOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
