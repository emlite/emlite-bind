use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserverSize {
    inner: emlite::Val,
}
impl FromVal for ResizeObserverSize {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeObserverSize {
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
impl core::ops::Deref for ResizeObserverSize {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResizeObserverSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ResizeObserverSize {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ResizeObserverSize {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ResizeObserverSize> for emlite::Val {
    fn from(s: ResizeObserverSize) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ResizeObserverSize);

impl ResizeObserverSize {
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }
}
impl ResizeObserverSize {
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }
}
