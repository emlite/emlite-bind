use super::*;

/// The PageSwapEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageSwapEventInit {
    inner: Any,
}

impl FromVal for PageSwapEventInit {
    fn from_val(v: &Any) -> Self {
        PageSwapEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PageSwapEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PageSwapEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PageSwapEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PageSwapEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PageSwapEventInit> for Any {
    fn from(s: PageSwapEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PageSwapEventInit> for Any {
    fn from(s: &PageSwapEventInit) -> Any {
        s.inner.clone()
    }
}

impl PageSwapEventInit {
    /// Getter of the `activation` attribute.
    pub fn activation(&self) -> NavigationActivation {
        self.inner.get("activation").as_::<NavigationActivation>()
    }

    /// Setter of the `activation` attribute.
    pub fn set_activation(&mut self, value: &NavigationActivation) {
        self.inner.set("activation", value);
    }
}
impl PageSwapEventInit {
    /// Getter of the `viewTransition` attribute.
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }

    /// Setter of the `viewTransition` attribute.
    pub fn set_view_transition(&mut self, value: &ViewTransition) {
        self.inner.set("viewTransition", value);
    }
}
