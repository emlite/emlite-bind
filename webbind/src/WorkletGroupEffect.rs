use super::*;

#[derive(Clone, Debug)]
pub struct WorkletGroupEffect {
    inner: emlite::Val,
}
impl FromVal for WorkletGroupEffect {
    fn from_val(v: &emlite::Val) -> Self {
        WorkletGroupEffect {
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
impl std::ops::Deref for WorkletGroupEffect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WorkletGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WorkletGroupEffect> for emlite::Val {
    fn from(s: WorkletGroupEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WorkletGroupEffect {
    pub fn get_children(&self) -> jsbind::Sequence<WorkletAnimationEffect> {
        self.inner
            .call("getChildren", &[])
            .as_::<jsbind::Sequence<WorkletAnimationEffect>>()
    }
}
