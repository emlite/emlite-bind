use super::*;

/// The GPUShaderModuleCompilationHint dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUShaderModuleCompilationHint {
    inner: Any,
}

impl FromVal for GPUShaderModuleCompilationHint {
    fn from_val(v: &Any) -> Self {
        GPUShaderModuleCompilationHint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUShaderModuleCompilationHint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUShaderModuleCompilationHint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUShaderModuleCompilationHint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUShaderModuleCompilationHint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUShaderModuleCompilationHint> for Any {
    fn from(s: GPUShaderModuleCompilationHint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUShaderModuleCompilationHint> for Any {
    fn from(s: &GPUShaderModuleCompilationHint) -> Any {
        s.inner.clone()
    }
}

impl GPUShaderModuleCompilationHint {
    /// Getter of the `entryPoint` attribute.
    pub fn entry_point(&self) -> JsString {
        self.inner.get("entryPoint").as_::<JsString>()
    }

    /// Setter of the `entryPoint` attribute.
    pub fn set_entry_point(&mut self, value: &JsString) {
        self.inner.set("entryPoint", value);
    }
}
impl GPUShaderModuleCompilationHint {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> Any {
        self.inner.get("layout").as_::<Any>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &Any) {
        self.inner.set("layout", value);
    }
}
