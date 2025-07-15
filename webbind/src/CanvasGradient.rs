use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CanvasGradient {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CanvasGradient {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanvasGradient {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CanvasGradient> for emlite::Val {
    fn from(s: CanvasGradient) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CanvasGradient> for emlite::Val {
    fn from(s: &CanvasGradient) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanvasGradient);

impl CanvasGradient {
    pub fn add_color_stop(&self, offset: f64, color: DOMString) -> Undefined {
        self.inner
            .call("addColorStop", &[offset.into(), color.into()])
            .as_::<Undefined>()
    }
}
