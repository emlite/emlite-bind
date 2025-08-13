use super::*;




/// The ModuleExportDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ModuleExportDescriptor {
    inner: Any,
}

impl FromVal for ModuleExportDescriptor {
    fn from_val(v: &Any) -> Self {
        ModuleExportDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ModuleExportDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ModuleExportDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ModuleExportDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ModuleExportDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ModuleExportDescriptor> for Any {
    fn from(s: ModuleExportDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ModuleExportDescriptor> for Any {
    fn from(s: &ModuleExportDescriptor) -> Any {
        s.inner.clone()
    }
}

impl ModuleExportDescriptor {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl ModuleExportDescriptor {
    /// Getter of the `kind` attribute.
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    /// Setter of the `kind` attribute.
    pub fn set_kind(&mut self, value: &ImportExportKind) {
        self.inner.set("kind", value);
    }
}
