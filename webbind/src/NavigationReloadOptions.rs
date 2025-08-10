use super::*;

/// The NavigationReloadOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationReloadOptions {
    inner: Any,
}

impl FromVal for NavigationReloadOptions {
    fn from_val(v: &Any) -> Self {
        NavigationReloadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationReloadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationReloadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationReloadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationReloadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationReloadOptions> for Any {
    fn from(s: NavigationReloadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationReloadOptions> for Any {
    fn from(s: &NavigationReloadOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationReloadOptions {
    /// Getter of the `state` attribute.
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    /// Setter of the `state` attribute.
    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
