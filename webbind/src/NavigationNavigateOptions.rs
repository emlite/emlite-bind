use super::*;

/// The NavigationNavigateOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationNavigateOptions {
    inner: Any,
}

impl FromVal for NavigationNavigateOptions {
    fn from_val(v: &Any) -> Self {
        NavigationNavigateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationNavigateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationNavigateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationNavigateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationNavigateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationNavigateOptions> for Any {
    fn from(s: NavigationNavigateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationNavigateOptions> for Any {
    fn from(s: &NavigationNavigateOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationNavigateOptions {
    /// Getter of the `state` attribute.
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    /// Setter of the `state` attribute.
    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
impl NavigationNavigateOptions {
    /// Getter of the `history` attribute.
    pub fn history(&self) -> NavigationHistoryBehavior {
        self.inner.get("history").as_::<NavigationHistoryBehavior>()
    }

    /// Setter of the `history` attribute.
    pub fn set_history(&mut self, value: &NavigationHistoryBehavior) {
        self.inner.set("history", value);
    }
}
