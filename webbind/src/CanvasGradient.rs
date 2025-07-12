use super::*;

#[derive(Clone, Debug)]
pub struct CanvasGradient {
    inner: emlite::Val,
}
impl FromVal for CanvasGradient {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasGradient {
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
impl std::ops::Deref for CanvasGradient {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasGradient> for emlite::Val {
    fn from(s: CanvasGradient) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasGradient {
    pub fn add_color_stop(&self, offset: f64, color: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("addColorStop", &[offset.into(), color.into()])
            .as_::<jsbind::Undefined>()
    }
}
