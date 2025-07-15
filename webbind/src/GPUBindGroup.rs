use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroup {
    inner: emlite::Val,
}
impl FromVal for GPUBindGroup {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBindGroup {
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
impl core::ops::Deref for GPUBindGroup {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUBindGroup {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUBindGroup {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUBindGroup> for emlite::Val {
    fn from(s: GPUBindGroup) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUBindGroup);

impl GPUBindGroup {
    pub fn label(&self) -> USVString {
        self.inner.get("label").as_::<USVString>()
    }

    pub fn set_label(&mut self, value: USVString) {
        self.inner.set("label", value);
    }
}
