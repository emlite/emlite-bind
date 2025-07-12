use super::*;

#[derive(Clone, Debug)]
pub struct MLGraph {
    inner: emlite::Val,
}
impl FromVal for MLGraph {
    fn from_val(v: &emlite::Val) -> Self {
        MLGraph {
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
impl std::ops::Deref for MLGraph {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGraph> for emlite::Val {
    fn from(s: MLGraph) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGraph {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
