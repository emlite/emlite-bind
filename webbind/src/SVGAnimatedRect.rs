use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedRect {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedRect {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedRect {
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
impl core::ops::Deref for SVGAnimatedRect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedRect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAnimatedRect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAnimatedRect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGAnimatedRect> for emlite::Val {
    fn from(s: SVGAnimatedRect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedRect);

impl SVGAnimatedRect {
    pub fn base_val(&self) -> DOMRect {
        self.inner.get("baseVal").as_::<DOMRect>()
    }
}
impl SVGAnimatedRect {
    pub fn anim_val(&self) -> DOMRectReadOnly {
        self.inner.get("animVal").as_::<DOMRectReadOnly>()
    }
}
