use super::*;




/// The ToggleEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ToggleEventInit {
    inner: Any,
}

impl FromVal for ToggleEventInit {
    fn from_val(v: &Any) -> Self {
        ToggleEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ToggleEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ToggleEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ToggleEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ToggleEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ToggleEventInit> for Any {
    fn from(s: ToggleEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ToggleEventInit> for Any {
    fn from(s: &ToggleEventInit) -> Any {
        s.inner.clone()
    }
}

impl ToggleEventInit {
    /// Getter of the `oldState` attribute.
    pub fn old_state(&self) -> JsString {
        self.inner.get("oldState").as_::<JsString>()
    }

    /// Setter of the `oldState` attribute.
    pub fn set_old_state(&mut self, value: &JsString) {
        self.inner.set("oldState", value);
    }
}
impl ToggleEventInit {
    /// Getter of the `newState` attribute.
    pub fn new_state(&self) -> JsString {
        self.inner.get("newState").as_::<JsString>()
    }

    /// Setter of the `newState` attribute.
    pub fn set_new_state(&mut self, value: &JsString) {
        self.inner.set("newState", value);
    }
}
