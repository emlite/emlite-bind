use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimatedPreserveAspectRatio {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedPreserveAspectRatio {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedPreserveAspectRatio {
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
impl std::ops::Deref for SVGAnimatedPreserveAspectRatio {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimatedPreserveAspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedPreserveAspectRatio> for emlite::Val {
    fn from(s: SVGAnimatedPreserveAspectRatio) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedPreserveAspectRatio {
    pub fn base_val(&self) -> SVGPreserveAspectRatio {
        self.inner.get("baseVal").as_::<SVGPreserveAspectRatio>()
    }
}
impl SVGAnimatedPreserveAspectRatio {
    pub fn anim_val(&self) -> SVGPreserveAspectRatio {
        self.inner.get("animVal").as_::<SVGPreserveAspectRatio>()
    }
}
