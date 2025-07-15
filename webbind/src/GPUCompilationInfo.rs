use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCompilationInfo {
    inner: emlite::Val,
}
impl FromVal for GPUCompilationInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCompilationInfo {
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
impl core::ops::Deref for GPUCompilationInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCompilationInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCompilationInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCompilationInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCompilationInfo> for emlite::Val {
    fn from(s: GPUCompilationInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUCompilationInfo);

impl GPUCompilationInfo {
    pub fn messages(&self) -> FrozenArray<GPUCompilationMessage> {
        self.inner
            .get("messages")
            .as_::<FrozenArray<GPUCompilationMessage>>()
    }
}
