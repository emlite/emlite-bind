use super::*;

/// The NavigationResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationResult {
    inner: Any,
}

impl FromVal for NavigationResult {
    fn from_val(v: &Any) -> Self {
        NavigationResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationResult> for Any {
    fn from(s: NavigationResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationResult> for Any {
    fn from(s: &NavigationResult) -> Any {
        s.inner.clone()
    }
}

impl NavigationResult {
    /// Getter of the `committed` attribute.
    pub fn committed(&self) -> Promise<NavigationHistoryEntry> {
        self.inner
            .get("committed")
            .as_::<Promise<NavigationHistoryEntry>>()
    }

    /// Setter of the `committed` attribute.
    pub fn set_committed(&mut self, value: &Promise<NavigationHistoryEntry>) {
        self.inner.set("committed", value);
    }
}
impl NavigationResult {
    /// Getter of the `finished` attribute.
    pub fn finished(&self) -> Promise<NavigationHistoryEntry> {
        self.inner
            .get("finished")
            .as_::<Promise<NavigationHistoryEntry>>()
    }

    /// Setter of the `finished` attribute.
    pub fn set_finished(&mut self, value: &Promise<NavigationHistoryEntry>) {
        self.inner.set("finished", value);
    }
}
