use super::*;




/// The CompositionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CompositionEventInit {
    inner: Any,
}

impl FromVal for CompositionEventInit {
    fn from_val(v: &Any) -> Self {
        CompositionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CompositionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CompositionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CompositionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CompositionEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CompositionEventInit> for Any {
    fn from(s: CompositionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CompositionEventInit> for Any {
    fn from(s: &CompositionEventInit) -> Any {
        s.inner.clone()
    }
}

impl CompositionEventInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &JsString) {
        self.inner.set("data", value);
    }
}
