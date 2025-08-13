use super::*;




/// The ModuleImportDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ModuleImportDescriptor {
    inner: Any,
}

impl FromVal for ModuleImportDescriptor {
    fn from_val(v: &Any) -> Self {
        ModuleImportDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ModuleImportDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ModuleImportDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ModuleImportDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ModuleImportDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ModuleImportDescriptor> for Any {
    fn from(s: ModuleImportDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ModuleImportDescriptor> for Any {
    fn from(s: &ModuleImportDescriptor) -> Any {
        s.inner.clone()
    }
}

impl ModuleImportDescriptor {
    /// Getter of the `module` attribute.
    pub fn module(&self) -> JsString {
        self.inner.get("module").as_::<JsString>()
    }

    /// Setter of the `module` attribute.
    pub fn set_module(&mut self, value: &JsString) {
        self.inner.set("module", value);
    }
}
impl ModuleImportDescriptor {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl ModuleImportDescriptor {
    /// Getter of the `kind` attribute.
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    /// Setter of the `kind` attribute.
    pub fn set_kind(&mut self, value: &ImportExportKind) {
        self.inner.set("kind", value);
    }
}
