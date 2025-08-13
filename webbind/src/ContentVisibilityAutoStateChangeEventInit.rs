use super::*;




/// The ContentVisibilityAutoStateChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentVisibilityAutoStateChangeEventInit {
    inner: Any,
}

impl FromVal for ContentVisibilityAutoStateChangeEventInit {
    fn from_val(v: &Any) -> Self {
        ContentVisibilityAutoStateChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ContentVisibilityAutoStateChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContentVisibilityAutoStateChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContentVisibilityAutoStateChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContentVisibilityAutoStateChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ContentVisibilityAutoStateChangeEventInit> for Any {
    fn from(s: ContentVisibilityAutoStateChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContentVisibilityAutoStateChangeEventInit> for Any {
    fn from(s: &ContentVisibilityAutoStateChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl ContentVisibilityAutoStateChangeEventInit {
    /// Getter of the `skipped` attribute.
    pub fn skipped(&self) -> bool {
        self.inner.get("skipped").as_::<bool>()
    }

    /// Setter of the `skipped` attribute.
    pub fn set_skipped(&mut self, value: bool) {
        self.inner.set("skipped", value);
    }
}
