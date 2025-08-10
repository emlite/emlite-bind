use super::*;

/// The NavigationPreloadState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationPreloadState {
    inner: Any,
}

impl FromVal for NavigationPreloadState {
    fn from_val(v: &Any) -> Self {
        NavigationPreloadState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationPreloadState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationPreloadState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationPreloadState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationPreloadState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationPreloadState> for Any {
    fn from(s: NavigationPreloadState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationPreloadState> for Any {
    fn from(s: &NavigationPreloadState) -> Any {
        s.inner.clone()
    }
}

impl NavigationPreloadState {
    /// Getter of the `enabled` attribute.
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    /// Setter of the `enabled` attribute.
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl NavigationPreloadState {
    /// Getter of the `headerValue` attribute.
    pub fn header_value(&self) -> JsString {
        self.inner.get("headerValue").as_::<JsString>()
    }

    /// Setter of the `headerValue` attribute.
    pub fn set_header_value(&mut self, value: &JsString) {
        self.inner.set("headerValue", value);
    }
}
