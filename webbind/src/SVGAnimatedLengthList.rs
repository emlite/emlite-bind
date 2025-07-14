use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGAnimatedLengthList {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedLengthList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedLengthList {
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
impl core::ops::Deref for SVGAnimatedLengthList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedLengthList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedLengthList> for emlite::Val {
    fn from(s: SVGAnimatedLengthList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedLengthList {
    pub fn base_val(&self) -> SVGLengthList {
        self.inner.get("baseVal").as_::<SVGLengthList>()
    }
}
impl SVGAnimatedLengthList {
    pub fn anim_val(&self) -> SVGLengthList {
        self.inner.get("animVal").as_::<SVGLengthList>()
    }
}
