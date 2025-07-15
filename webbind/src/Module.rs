use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ModuleExportDescriptor {
    inner: emlite::Val,
}
impl FromVal for ModuleExportDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        ModuleExportDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ModuleExportDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ModuleExportDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ModuleExportDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ModuleExportDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ModuleExportDescriptor> for emlite::Val {
    fn from(s: ModuleExportDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ModuleExportDescriptor {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl ModuleExportDescriptor {
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    pub fn set_kind(&mut self, value: ImportExportKind) {
        self.inner.set("kind", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ModuleImportDescriptor {
    inner: emlite::Val,
}
impl FromVal for ModuleImportDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        ModuleImportDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ModuleImportDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ModuleImportDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ModuleImportDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ModuleImportDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ModuleImportDescriptor> for emlite::Val {
    fn from(s: ModuleImportDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ModuleImportDescriptor {
    pub fn module(&self) -> USVString {
        self.inner.get("module").as_::<USVString>()
    }

    pub fn set_module(&mut self, value: USVString) {
        self.inner.set("module", value);
    }

}
impl ModuleImportDescriptor {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl ModuleImportDescriptor {
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    pub fn set_kind(&mut self, value: ImportExportKind) {
        self.inner.set("kind", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Module {
    inner: emlite::Val,
}
impl FromVal for Module {
    fn from_val(v: &emlite::Val) -> Self {
        Module { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Module {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Module {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Module {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Module> for emlite::Val {
    fn from(s: Module) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Module);



impl Module {
    pub fn new(bytes: Any) -> Module {
        Self {
            inner: emlite::Val::global("Module").new(&[bytes.into()]).as_::<emlite::Val>(),
        }
    }

}
impl Module {
    pub fn exports(module_object: Module) -> Sequence<ModuleExportDescriptor> {
        emlite::Val::global("module").call("exports", &[module_object.into(), ]).as_::<Sequence<ModuleExportDescriptor>>()
    }

}
impl Module {
    pub fn imports(module_object: Module) -> Sequence<ModuleImportDescriptor> {
        emlite::Val::global("module").call("imports", &[module_object.into(), ]).as_::<Sequence<ModuleImportDescriptor>>()
    }

}
impl Module {
    pub fn custom_sections(module_object: Module, section_name: DOMString) -> Sequence<ArrayBuffer> {
        emlite::Val::global("module").call("customSections", &[module_object.into(), section_name.into(), ]).as_::<Sequence<ArrayBuffer>>()
    }

}
