use super::*;

/// The WorkletGroupEffect class.
/// [`WorkletGroupEffect`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletGroupEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletGroupEffect {
    inner: Any,
}

impl FromVal for WorkletGroupEffect {
    fn from_val(v: &Any) -> Self {
        WorkletGroupEffect {
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

impl core::ops::Deref for WorkletGroupEffect {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkletGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkletGroupEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkletGroupEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkletGroupEffect> for Any {
    fn from(s: WorkletGroupEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkletGroupEffect> for Any {
    fn from(s: &WorkletGroupEffect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkletGroupEffect);

impl WorkletGroupEffect {
    /// The getChildren method.
    /// [`WorkletGroupEffect.getChildren`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletGroupEffect/getChildren)
    pub fn get_children(&self) -> TypedArray<WorkletAnimationEffect> {
        self.inner
            .call("getChildren", &[])
            .as_::<TypedArray<WorkletAnimationEffect>>()
    }
}
