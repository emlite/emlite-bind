use super::*;




/// The PageRevealEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageRevealEventInit {
    inner: Any,
}

impl FromVal for PageRevealEventInit {
    fn from_val(v: &Any) -> Self {
        PageRevealEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PageRevealEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PageRevealEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PageRevealEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PageRevealEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PageRevealEventInit> for Any {
    fn from(s: PageRevealEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PageRevealEventInit> for Any {
    fn from(s: &PageRevealEventInit) -> Any {
        s.inner.clone()
    }
}

impl PageRevealEventInit {
    /// Getter of the `viewTransition` attribute.
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }

    /// Setter of the `viewTransition` attribute.
    pub fn set_view_transition(&mut self, value: &ViewTransition) {
        self.inner.set("viewTransition", value);
    }
}
