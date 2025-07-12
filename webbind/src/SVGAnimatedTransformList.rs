use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimatedTransformList {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedTransformList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedTransformList {
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
impl std::ops::Deref for SVGAnimatedTransformList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimatedTransformList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedTransformList> for emlite::Val {
    fn from(s: SVGAnimatedTransformList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedTransformList {
    pub fn base_val(&self) -> SVGTransformList {
        self.inner.get("baseVal").as_::<SVGTransformList>()
    }
}
impl SVGAnimatedTransformList {
    pub fn anim_val(&self) -> SVGTransformList {
        self.inner.get("animVal").as_::<SVGTransformList>()
    }
}
