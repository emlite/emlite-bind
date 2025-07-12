use super::*;

#[derive(Clone, Debug)]
pub struct GPUBindGroupLayout {
    inner: emlite::Val,
}
impl FromVal for GPUBindGroupLayout {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBindGroupLayout {
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
impl std::ops::Deref for GPUBindGroupLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUBindGroupLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUBindGroupLayout> for emlite::Val {
    fn from(s: GPUBindGroupLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBindGroupLayout {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
