use super::*;

#[derive(Clone, Debug)]
pub struct WorkletGlobalScope {
    inner: emlite::Val,
}
impl FromVal for WorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        WorkletGlobalScope {
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
impl std::ops::Deref for WorkletGlobalScope {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WorkletGlobalScope> for emlite::Val {
    fn from(s: WorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
