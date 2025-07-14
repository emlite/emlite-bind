use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CanvasPattern {
    inner: emlite::Val,
}
impl FromVal for CanvasPattern {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasPattern {
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
impl core::ops::Deref for CanvasPattern {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasPattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasPattern> for emlite::Val {
    fn from(s: CanvasPattern) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasPattern {
    pub fn set_transform0(&self) -> jsbind::Undefined {
        self.inner
            .call("setTransform", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_transform1(&self, transform: DOMMatrix2DInit) -> jsbind::Undefined {
        self.inner
            .call("setTransform", &[transform.into()])
            .as_::<jsbind::Undefined>()
    }
}
