use super::*;




/// The NavigationCurrentEntryChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationCurrentEntryChangeEventInit {
    inner: Any,
}

impl FromVal for NavigationCurrentEntryChangeEventInit {
    fn from_val(v: &Any) -> Self {
        NavigationCurrentEntryChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationCurrentEntryChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationCurrentEntryChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationCurrentEntryChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationCurrentEntryChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NavigationCurrentEntryChangeEventInit> for Any {
    fn from(s: NavigationCurrentEntryChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationCurrentEntryChangeEventInit> for Any {
    fn from(s: &NavigationCurrentEntryChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl NavigationCurrentEntryChangeEventInit {
    /// Getter of the `navigationType` attribute.
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }

    /// Setter of the `navigationType` attribute.
    pub fn set_navigation_type(&mut self, value: &NavigationType) {
        self.inner.set("navigationType", value);
    }
}
impl NavigationCurrentEntryChangeEventInit {
    /// Getter of the `from` attribute.
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }

    /// Setter of the `from` attribute.
    pub fn set_from(&mut self, value: &NavigationHistoryEntry) {
        self.inner.set("from", value);
    }
}
