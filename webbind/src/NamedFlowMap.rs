use super::*;

#[derive(Clone, Debug)]
pub struct NamedFlowMap {
    inner: emlite::Val,
}
impl FromVal for NamedFlowMap {
    fn from_val(v: &emlite::Val) -> Self {
        NamedFlowMap {
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
impl std::ops::Deref for NamedFlowMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NamedFlowMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NamedFlowMap> for emlite::Val {
    fn from(s: NamedFlowMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
