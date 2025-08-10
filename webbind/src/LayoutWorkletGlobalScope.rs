use super::*;

/// The LayoutWorkletGlobalScope class.
/// [`LayoutWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutWorkletGlobalScope {
    inner: WorkletGlobalScope,
}

impl FromVal for LayoutWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        LayoutWorkletGlobalScope {
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

impl core::ops::Deref for LayoutWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LayoutWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LayoutWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LayoutWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<LayoutWorkletGlobalScope> for Any {
    fn from(s: LayoutWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LayoutWorkletGlobalScope> for Any {
    fn from(s: &LayoutWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LayoutWorkletGlobalScope);

impl LayoutWorkletGlobalScope {
    /// The registerLayout method.
    /// [`LayoutWorkletGlobalScope.registerLayout`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutWorkletGlobalScope/registerLayout)
    pub fn register_layout(&self, name: &JsString, layout_ctor: &Function) -> Undefined {
        self.inner
            .call("registerLayout", &[name.into(), layout_ctor.into()])
            .as_::<Undefined>()
    }
}
