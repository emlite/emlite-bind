use super::*;

/// The FullscreenOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FullscreenOptions {
    inner: Any,
}

impl FromVal for FullscreenOptions {
    fn from_val(v: &Any) -> Self {
        FullscreenOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FullscreenOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FullscreenOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FullscreenOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FullscreenOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FullscreenOptions> for Any {
    fn from(s: FullscreenOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FullscreenOptions> for Any {
    fn from(s: &FullscreenOptions) -> Any {
        s.inner.clone()
    }
}

impl FullscreenOptions {
    /// Getter of the `screen` attribute.
    pub fn screen(&self) -> ScreenDetailed {
        self.inner.get("screen").as_::<ScreenDetailed>()
    }

    /// Setter of the `screen` attribute.
    pub fn set_screen(&mut self, value: &ScreenDetailed) {
        self.inner.set("screen", value);
    }
}
