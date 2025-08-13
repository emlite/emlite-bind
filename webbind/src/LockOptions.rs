use super::*;




/// The LockOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockOptions {
    inner: Any,
}

impl FromVal for LockOptions {
    fn from_val(v: &Any) -> Self {
        LockOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LockOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LockOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LockOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LockOptions> for Any {
    fn from(s: LockOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LockOptions> for Any {
    fn from(s: &LockOptions) -> Any {
        s.inner.clone()
    }
}

impl LockOptions {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> LockMode {
        self.inner.get("mode").as_::<LockMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &LockMode) {
        self.inner.set("mode", value);
    }
}
impl LockOptions {
    /// Getter of the `ifAvailable` attribute.
    pub fn if_available(&self) -> bool {
        self.inner.get("ifAvailable").as_::<bool>()
    }

    /// Setter of the `ifAvailable` attribute.
    pub fn set_if_available(&mut self, value: bool) {
        self.inner.set("ifAvailable", value);
    }
}
impl LockOptions {
    /// Getter of the `steal` attribute.
    pub fn steal(&self) -> bool {
        self.inner.get("steal").as_::<bool>()
    }

    /// Setter of the `steal` attribute.
    pub fn set_steal(&mut self, value: bool) {
        self.inner.set("steal", value);
    }
}
impl LockOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
