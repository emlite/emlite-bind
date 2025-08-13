use super::*;




/// The GPUShaderModuleDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUShaderModuleDescriptor {
    inner: Any,
}

impl FromVal for GPUShaderModuleDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUShaderModuleDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUShaderModuleDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUShaderModuleDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUShaderModuleDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUShaderModuleDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUShaderModuleDescriptor> for Any {
    fn from(s: GPUShaderModuleDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUShaderModuleDescriptor> for Any {
    fn from(s: &GPUShaderModuleDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUShaderModuleDescriptor {
    /// Getter of the `code` attribute.
    pub fn code(&self) -> JsString {
        self.inner.get("code").as_::<JsString>()
    }

    /// Setter of the `code` attribute.
    pub fn set_code(&mut self, value: &JsString) {
        self.inner.set("code", value);
    }
}
impl GPUShaderModuleDescriptor {
    /// Getter of the `compilationHints` attribute.
    pub fn compilation_hints(&self) -> TypedArray<GPUShaderModuleCompilationHint> {
        self.inner.get("compilationHints").as_::<TypedArray<GPUShaderModuleCompilationHint>>()
    }

    /// Setter of the `compilationHints` attribute.
    pub fn set_compilation_hints(&mut self, value: &TypedArray<GPUShaderModuleCompilationHint>) {
        self.inner.set("compilationHints", value);
    }
}
