use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaintWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for PaintWorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        PaintWorkletGlobalScope {
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
impl core::ops::Deref for PaintWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaintWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PaintWorkletGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaintWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PaintWorkletGlobalScope> for emlite::Val {
    fn from(s: PaintWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PaintWorkletGlobalScope);

impl PaintWorkletGlobalScope {
    pub fn register_paint(
        &self,
        name: jsbind::DOMString,
        paint_ctor: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("registerPaint", &[name.into(), paint_ctor.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl PaintWorkletGlobalScope {
    pub fn device_pixel_ratio(&self) -> f64 {
        self.inner.get("devicePixelRatio").as_::<f64>()
    }
}
