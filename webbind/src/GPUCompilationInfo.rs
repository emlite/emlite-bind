use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for GPUCompilationInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCompilationInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCompilationInfo> for emlite::Val {
    fn from(s: GPUCompilationInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCompilationInfo {
    pub fn messages(&self) -> jsbind::FrozenArray<GPUCompilationMessage> {
        self.inner
            .get("messages")
            .as_::<jsbind::FrozenArray<GPUCompilationMessage>>()
    }
}
