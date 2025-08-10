use super::*;

/// The PADebugModeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PADebugModeOptions {
    inner: Any,
}

impl FromVal for PADebugModeOptions {
    fn from_val(v: &Any) -> Self {
        PADebugModeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PADebugModeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PADebugModeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PADebugModeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PADebugModeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PADebugModeOptions> for Any {
    fn from(s: PADebugModeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PADebugModeOptions> for Any {
    fn from(s: &PADebugModeOptions) -> Any {
        s.inner.clone()
    }
}

impl PADebugModeOptions {
    /// Getter of the `debugKey` attribute.
    pub fn debug_key(&self) -> i64 {
        self.inner.get("debugKey").as_::<i64>()
    }

    /// Setter of the `debugKey` attribute.
    pub fn set_debug_key(&mut self, value: i64) {
        self.inner.set("debugKey", value);
    }
}
