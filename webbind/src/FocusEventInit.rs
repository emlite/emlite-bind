use super::*;




/// The FocusEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusEventInit {
    inner: Any,
}

impl FromVal for FocusEventInit {
    fn from_val(v: &Any) -> Self {
        FocusEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FocusEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FocusEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FocusEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FocusEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FocusEventInit> for Any {
    fn from(s: FocusEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FocusEventInit> for Any {
    fn from(s: &FocusEventInit) -> Any {
        s.inner.clone()
    }
}

impl FocusEventInit {
    /// Getter of the `relatedTarget` attribute.
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }

    /// Setter of the `relatedTarget` attribute.
    pub fn set_related_target(&mut self, value: &EventTarget) {
        self.inner.set("relatedTarget", value);
    }
}
