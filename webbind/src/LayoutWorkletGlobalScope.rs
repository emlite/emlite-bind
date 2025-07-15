use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for LayoutWorkletGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LayoutWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&LayoutWorkletGlobalScope> for emlite::Val {
    fn from(s: &LayoutWorkletGlobalScope) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutWorkletGlobalScope);

impl LayoutWorkletGlobalScope {
    pub fn register_layout(&self, name: DOMString, layout_ctor: Any) -> Undefined {
        self.inner
            .call("registerLayout", &[name.into(), layout_ctor.into()])
            .as_::<Undefined>()
    }
}
