use super::*;

/// The NavigationUpdateCurrentEntryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationUpdateCurrentEntryOptions {
    inner: Any,
}

impl FromVal for NavigationUpdateCurrentEntryOptions {
    fn from_val(v: &Any) -> Self {
        NavigationUpdateCurrentEntryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationUpdateCurrentEntryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationUpdateCurrentEntryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationUpdateCurrentEntryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationUpdateCurrentEntryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationUpdateCurrentEntryOptions> for Any {
    fn from(s: NavigationUpdateCurrentEntryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationUpdateCurrentEntryOptions> for Any {
    fn from(s: &NavigationUpdateCurrentEntryOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationUpdateCurrentEntryOptions {
    /// Getter of the `state` attribute.
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    /// Setter of the `state` attribute.
    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
