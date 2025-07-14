use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LayoutWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for LayoutWorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutWorkletGlobalScope {
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
impl From<LayoutWorkletGlobalScope> for emlite::Val {
    fn from(s: LayoutWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutWorkletGlobalScope {
    pub fn register_layout(
        &self,
        name: jsbind::DOMString,
        layout_ctor: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("registerLayout", &[name.into(), layout_ctor.into()])
            .as_::<jsbind::Undefined>()
    }
}
