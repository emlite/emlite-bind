use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AnimationWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for AnimationWorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationWorkletGlobalScope> for emlite::Val {
    fn from(s: AnimationWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationWorkletGlobalScope {
    pub fn register_animator(
        &self,
        name: jsbind::DOMString,
        animator_ctor: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call("registerAnimator", &[name.into(), animator_ctor.into()])
            .as_::<jsbind::Undefined>()
    }
}
