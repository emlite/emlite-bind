use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimatedNumberList {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedNumberList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedNumberList {
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
impl std::ops::Deref for SVGAnimatedNumberList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimatedNumberList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedNumberList> for emlite::Val {
    fn from(s: SVGAnimatedNumberList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedNumberList {
    pub fn base_val(&self) -> SVGNumberList {
        self.inner.get("baseVal").as_::<SVGNumberList>()
    }
}
impl SVGAnimatedNumberList {
    pub fn anim_val(&self) -> SVGNumberList {
        self.inner.get("animVal").as_::<SVGNumberList>()
    }
}
