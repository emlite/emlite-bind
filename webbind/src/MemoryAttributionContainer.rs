use super::*;




/// The MemoryAttributionContainer dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryAttributionContainer {
    inner: Any,
}

impl FromVal for MemoryAttributionContainer {
    fn from_val(v: &Any) -> Self {
        MemoryAttributionContainer { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MemoryAttributionContainer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MemoryAttributionContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MemoryAttributionContainer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MemoryAttributionContainer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MemoryAttributionContainer> for Any {
    fn from(s: MemoryAttributionContainer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MemoryAttributionContainer> for Any {
    fn from(s: &MemoryAttributionContainer) -> Any {
        s.inner.clone()
    }
}

impl MemoryAttributionContainer {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl MemoryAttributionContainer {
    /// Getter of the `src` attribute.
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
