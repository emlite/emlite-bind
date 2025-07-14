use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUTextureView {
    inner: emlite::Val,
}
impl FromVal for GPUTextureView {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureView {
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
impl core::ops::Deref for GPUTextureView {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTextureView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureView> for emlite::Val {
    fn from(s: GPUTextureView) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureView {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
