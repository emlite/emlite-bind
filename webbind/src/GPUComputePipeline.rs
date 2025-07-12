use super::*;

#[derive(Clone, Debug)]
pub struct GPUComputePipeline {
    inner: emlite::Val,
}
impl FromVal for GPUComputePipeline {
    fn from_val(v: &emlite::Val) -> Self {
        GPUComputePipeline {
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
impl std::ops::Deref for GPUComputePipeline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUComputePipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUComputePipeline> for emlite::Val {
    fn from(s: GPUComputePipeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUComputePipeline {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
impl GPUComputePipeline {
    pub fn get_bind_group_layout(&self, index: u32) -> GPUBindGroupLayout {
        self.inner
            .call("getBindGroupLayout", &[index.into()])
            .as_::<GPUBindGroupLayout>()
    }
}
