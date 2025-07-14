use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGAnimatedLength {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedLength {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedLength {
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
impl core::ops::Deref for SVGAnimatedLength {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedLength> for emlite::Val {
    fn from(s: SVGAnimatedLength) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedLength {
    pub fn base_val(&self) -> SVGLength {
        self.inner.get("baseVal").as_::<SVGLength>()
    }
}
impl SVGAnimatedLength {
    pub fn anim_val(&self) -> SVGLength {
        self.inner.get("animVal").as_::<SVGLength>()
    }
}
