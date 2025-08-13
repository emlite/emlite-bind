use super::*;




/// The ImportNodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImportNodeOptions {
    inner: Any,
}

impl FromVal for ImportNodeOptions {
    fn from_val(v: &Any) -> Self {
        ImportNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImportNodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImportNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImportNodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImportNodeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImportNodeOptions> for Any {
    fn from(s: ImportNodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImportNodeOptions> for Any {
    fn from(s: &ImportNodeOptions) -> Any {
        s.inner.clone()
    }
}

impl ImportNodeOptions {
    /// Getter of the `customElementRegistry` attribute.
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner.get("customElementRegistry").as_::<CustomElementRegistry>()
    }

    /// Setter of the `customElementRegistry` attribute.
    pub fn set_custom_element_registry(&mut self, value: &CustomElementRegistry) {
        self.inner.set("customElementRegistry", value);
    }
}
impl ImportNodeOptions {
    /// Getter of the `selfOnly` attribute.
    pub fn self_only(&self) -> bool {
        self.inner.get("selfOnly").as_::<bool>()
    }

    /// Setter of the `selfOnly` attribute.
    pub fn set_self_only(&mut self, value: bool) {
        self.inner.set("selfOnly", value);
    }
}
