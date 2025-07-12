use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for PaintWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaintWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaintWorkletGlobalScope> for emlite::Val {
    fn from(s: PaintWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
