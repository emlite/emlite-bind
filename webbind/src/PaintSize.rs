use super::*;

#[derive(Clone, Debug)]
pub struct PaintSize {
    inner: emlite::Val,
}
impl FromVal for PaintSize {
    fn from_val(v: &emlite::Val) -> Self {
        PaintSize {
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
impl std::ops::Deref for PaintSize {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaintSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaintSize> for emlite::Val {
    fn from(s: PaintSize) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaintSize {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl PaintSize {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
