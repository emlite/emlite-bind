use super::*;

/// The WorkletGlobalScope class.
/// [`WorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletGlobalScope {
    inner: Any,
}

impl FromVal for WorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        WorkletGlobalScope {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WorkletGlobalScope {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkletGlobalScope> for Any {
    fn from(s: WorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkletGlobalScope> for Any {
    fn from(s: &WorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkletGlobalScope);
