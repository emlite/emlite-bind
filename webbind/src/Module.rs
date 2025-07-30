use super::*;

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
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl ModuleExportDescriptor {
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    pub fn set_kind(&mut self, value: &ImportExportKind) {
        self.inner.set("kind", value);
    }
}
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
    pub fn module(&self) -> JsString {
        self.inner.get("module").as_::<JsString>()
    }

    pub fn set_module(&mut self, value: &JsString) {
        self.inner.set("module", value);
    }
}
impl ModuleImportDescriptor {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl ModuleImportDescriptor {
    pub fn kind(&self) -> ImportExportKind {
        self.inner.get("kind").as_::<ImportExportKind>()
    }

    pub fn set_kind(&mut self, value: &ImportExportKind) {
        self.inner.set("kind", value);
    }
}
/// The Module class.
/// [`Module`](https://developer.mozilla.org/en-US/docs/Web/API/Module)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Module {
    inner: Any,
}
impl FromVal for Module {
    fn from_val(v: &Any) -> Self {
        Module {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Module {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Module {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Module {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Module> for Any {
    fn from(s: Module) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Module> for Any {
    fn from(s: &Module) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Module);

impl Module {
    /// The `new Module(..)` constructor, creating a new Module instance
    pub fn new(bytes: &Any) -> Module {
        Self {
            inner: Any::global("Module").new(&[bytes.into()]).as_::<Any>(),
        }
    }
}
impl Module {
    /// The exports method.
    /// [`Module.exports`](https://developer.mozilla.org/en-US/docs/Web/API/Module/exports)
    pub fn exports(module_object: &Module) -> TypedArray<ModuleExportDescriptor> {
        Any::global("Module")
            .call("exports", &[module_object.into()])
            .as_::<TypedArray<ModuleExportDescriptor>>()
    }
}
impl Module {
    /// The imports method.
    /// [`Module.imports`](https://developer.mozilla.org/en-US/docs/Web/API/Module/imports)
    pub fn imports(module_object: &Module) -> TypedArray<ModuleImportDescriptor> {
        Any::global("Module")
            .call("imports", &[module_object.into()])
            .as_::<TypedArray<ModuleImportDescriptor>>()
    }
}
impl Module {
    /// The customSections method.
    /// [`Module.customSections`](https://developer.mozilla.org/en-US/docs/Web/API/Module/customSections)
    pub fn custom_sections(
        module_object: &Module,
        section_name: &JsString,
    ) -> TypedArray<ArrayBuffer> {
        Any::global("Module")
            .call(
                "customSections",
                &[module_object.into(), section_name.into()],
            )
            .as_::<TypedArray<ArrayBuffer>>()
    }
}
