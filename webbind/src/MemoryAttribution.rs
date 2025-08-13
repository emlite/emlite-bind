use super::*;




/// The MemoryAttribution dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryAttribution {
    inner: Any,
}

impl FromVal for MemoryAttribution {
    fn from_val(v: &Any) -> Self {
        MemoryAttribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MemoryAttribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MemoryAttribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MemoryAttribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MemoryAttribution {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MemoryAttribution> for Any {
    fn from(s: MemoryAttribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MemoryAttribution> for Any {
    fn from(s: &MemoryAttribution) -> Any {
        s.inner.clone()
    }
}

impl MemoryAttribution {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl MemoryAttribution {
    /// Getter of the `container` attribute.
    pub fn container(&self) -> MemoryAttributionContainer {
        self.inner.get("container").as_::<MemoryAttributionContainer>()
    }

    /// Setter of the `container` attribute.
    pub fn set_container(&mut self, value: &MemoryAttributionContainer) {
        self.inner.set("container", value);
    }
}
impl MemoryAttribution {
    /// Getter of the `scope` attribute.
    pub fn scope(&self) -> JsString {
        self.inner.get("scope").as_::<JsString>()
    }

    /// Setter of the `scope` attribute.
    pub fn set_scope(&mut self, value: &JsString) {
        self.inner.set("scope", value);
    }
}
