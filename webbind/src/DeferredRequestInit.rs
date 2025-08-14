use super::*;




/// The DeferredRequestInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeferredRequestInit {
    inner: Any,
}

impl FromVal for DeferredRequestInit {
    fn from_val(v: &Any) -> Self {
        DeferredRequestInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeferredRequestInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeferredRequestInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeferredRequestInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeferredRequestInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DeferredRequestInit> for Any {
    fn from(s: DeferredRequestInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeferredRequestInit> for Any {
    fn from(s: &DeferredRequestInit) -> Any {
        s.inner.clone()
    }
}

impl DeferredRequestInit {
    /// Getter of the `activateAfter` attribute.
    pub fn activate_after(&self) -> Any {
        self.inner.get("activateAfter").as_::<Any>()
    }

    /// Setter of the `activateAfter` attribute.
    pub fn set_activate_after(&mut self, value: &Any) {
        self.inner.set("activateAfter", value);
    }
}
