use super::*;

/// The MLContextOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContextOptions {
    inner: Any,
}

impl FromVal for MLContextOptions {
    fn from_val(v: &Any) -> Self {
        MLContextOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLContextOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLContextOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLContextOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLContextOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLContextOptions> for Any {
    fn from(s: MLContextOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLContextOptions> for Any {
    fn from(s: &MLContextOptions) -> Any {
        s.inner.clone()
    }
}

impl MLContextOptions {
    /// Getter of the `powerPreference` attribute.
    pub fn power_preference(&self) -> MLPowerPreference {
        self.inner.get("powerPreference").as_::<MLPowerPreference>()
    }

    /// Setter of the `powerPreference` attribute.
    pub fn set_power_preference(&mut self, value: &MLPowerPreference) {
        self.inner.set("powerPreference", value);
    }
}
