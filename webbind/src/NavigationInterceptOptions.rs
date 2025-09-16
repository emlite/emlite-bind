use super::*;

/// The NavigationInterceptOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationInterceptOptions {
    inner: Any,
}

impl FromVal for NavigationInterceptOptions {
    fn from_val(v: &Any) -> Self {
        NavigationInterceptOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationInterceptOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationInterceptOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationInterceptOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationInterceptOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationInterceptOptions> for Any {
    fn from(s: NavigationInterceptOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationInterceptOptions> for Any {
    fn from(s: &NavigationInterceptOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationInterceptOptions {
    /// Getter of the `precommitHandler` attribute.
    pub fn precommit_handler(&self) -> Function {
        self.inner.get("precommitHandler").as_::<Function>()
    }

    /// Setter of the `precommitHandler` attribute.
    pub fn set_precommit_handler(&mut self, value: &Function) {
        self.inner.set("precommitHandler", value);
    }
}
impl NavigationInterceptOptions {
    /// Getter of the `handler` attribute.
    pub fn handler(&self) -> Function {
        self.inner.get("handler").as_::<Function>()
    }

    /// Setter of the `handler` attribute.
    pub fn set_handler(&mut self, value: &Function) {
        self.inner.set("handler", value);
    }
}
impl NavigationInterceptOptions {
    /// Getter of the `focusReset` attribute.
    pub fn focus_reset(&self) -> NavigationFocusReset {
        self.inner.get("focusReset").as_::<NavigationFocusReset>()
    }

    /// Setter of the `focusReset` attribute.
    pub fn set_focus_reset(&mut self, value: &NavigationFocusReset) {
        self.inner.set("focusReset", value);
    }
}
impl NavigationInterceptOptions {
    /// Getter of the `scroll` attribute.
    pub fn scroll(&self) -> NavigationScrollBehavior {
        self.inner.get("scroll").as_::<NavigationScrollBehavior>()
    }

    /// Setter of the `scroll` attribute.
    pub fn set_scroll(&mut self, value: &NavigationScrollBehavior) {
        self.inner.set("scroll", value);
    }
}
