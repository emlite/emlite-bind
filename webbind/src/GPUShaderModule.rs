use super::*;

#[derive(Clone, Debug)]
pub struct GPUShaderModule {
    inner: emlite::Val,
}
impl FromVal for GPUShaderModule {
    fn from_val(v: &emlite::Val) -> Self {
        GPUShaderModule {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUShaderModule {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUShaderModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUShaderModule> for emlite::Val {
    fn from(s: GPUShaderModule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUShaderModule {
    pub fn get_compilation_info(&self) -> jsbind::Promise {
        self.inner
            .call("getCompilationInfo", &[])
            .as_::<jsbind::Promise>()
    }
}
impl GPUShaderModule {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
