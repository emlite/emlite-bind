use super::*;

/// The AnimationWorkletGlobalScope class.
/// [`AnimationWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for AnimationWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        AnimationWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for AnimationWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AnimationWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AnimationWorkletGlobalScope> for Any {
    fn from(s: AnimationWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AnimationWorkletGlobalScope> for Any {
    fn from(s: &AnimationWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AnimationWorkletGlobalScope);

impl AnimationWorkletGlobalScope {
    /// The registerAnimator method.
    /// [`AnimationWorkletGlobalScope.registerAnimator`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationWorkletGlobalScope/registerAnimator)
    pub fn register_animator(&self, name: &JsString, animator_ctor: &Function) -> Undefined {
        self.inner
            .call("registerAnimator", &[name.into(), animator_ctor.into()])
            .as_::<Undefined>()
    }
}
