use super::*;

/// The PageTransitionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageTransitionEventInit {
    inner: Any,
}

impl FromVal for PageTransitionEventInit {
    fn from_val(v: &Any) -> Self {
        PageTransitionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PageTransitionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PageTransitionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PageTransitionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PageTransitionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PageTransitionEventInit> for Any {
    fn from(s: PageTransitionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PageTransitionEventInit> for Any {
    fn from(s: &PageTransitionEventInit) -> Any {
        s.inner.clone()
    }
}

impl PageTransitionEventInit {
    /// Getter of the `persisted` attribute.
    pub fn persisted(&self) -> bool {
        self.inner.get("persisted").as_::<bool>()
    }

    /// Setter of the `persisted` attribute.
    pub fn set_persisted(&mut self, value: bool) {
        self.inner.set("persisted", value);
    }
}
