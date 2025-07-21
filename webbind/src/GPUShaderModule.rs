use super::*;

/// The GPUShaderModule class.
/// [`GPUShaderModule`](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUShaderModule {
    inner: Any,
}
impl FromVal for GPUShaderModule {
    fn from_val(v: &Any) -> Self {
        GPUShaderModule {
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
impl core::ops::Deref for GPUShaderModule {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUShaderModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUShaderModule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUShaderModule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUShaderModule> for Any {
    fn from(s: GPUShaderModule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUShaderModule> for Any {
    fn from(s: &GPUShaderModule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUShaderModule);

impl GPUShaderModule {
    /// The getCompilationInfo method.
    /// [`GPUShaderModule.getCompilationInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule/getCompilationInfo)
    pub fn get_compilation_info(&self) -> Promise<GPUCompilationInfo> {
        self.inner
            .call("getCompilationInfo", &[])
            .as_::<Promise<GPUCompilationInfo>>()
    }
}
impl GPUShaderModule {
    /// Getter of the `label` attribute.
    /// [`GPUShaderModule.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule/label)
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUShaderModule.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule/label)
    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
