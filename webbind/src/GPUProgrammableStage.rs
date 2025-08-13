use super::*;




/// The GPUProgrammableStage dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUProgrammableStage {
    inner: Any,
}

impl FromVal for GPUProgrammableStage {
    fn from_val(v: &Any) -> Self {
        GPUProgrammableStage { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUProgrammableStage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUProgrammableStage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUProgrammableStage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUProgrammableStage {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUProgrammableStage> for Any {
    fn from(s: GPUProgrammableStage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUProgrammableStage> for Any {
    fn from(s: &GPUProgrammableStage) -> Any {
        s.inner.clone()
    }
}

impl GPUProgrammableStage {
    /// Getter of the `module` attribute.
    pub fn module(&self) -> GPUShaderModule {
        self.inner.get("module").as_::<GPUShaderModule>()
    }

    /// Setter of the `module` attribute.
    pub fn set_module(&mut self, value: &GPUShaderModule) {
        self.inner.set("module", value);
    }
}
impl GPUProgrammableStage {
    /// Getter of the `entryPoint` attribute.
    pub fn entry_point(&self) -> JsString {
        self.inner.get("entryPoint").as_::<JsString>()
    }

    /// Setter of the `entryPoint` attribute.
    pub fn set_entry_point(&mut self, value: &JsString) {
        self.inner.set("entryPoint", value);
    }
}
impl GPUProgrammableStage {
    /// Getter of the `constants` attribute.
    pub fn constants(&self) -> Record<JsString, Any> {
        self.inner.get("constants").as_::<Record<JsString, Any>>()
    }

    /// Setter of the `constants` attribute.
    pub fn set_constants(&mut self, value: &Record<JsString, Any>) {
        self.inner.set("constants", value);
    }
}
