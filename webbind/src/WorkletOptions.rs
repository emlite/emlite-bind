use super::*;

/// The WorkletOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletOptions {
    inner: Any,
}

impl FromVal for WorkletOptions {
    fn from_val(v: &Any) -> Self {
        WorkletOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WorkletOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkletOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkletOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkletOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkletOptions> for Any {
    fn from(s: WorkletOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkletOptions> for Any {
    fn from(s: &WorkletOptions) -> Any {
        s.inner.clone()
    }
}

impl WorkletOptions {
    /// Getter of the `credentials` attribute.
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }

    /// Setter of the `credentials` attribute.
    pub fn set_credentials(&mut self, value: &RequestCredentials) {
        self.inner.set("credentials", value);
    }
}
