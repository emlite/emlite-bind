use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimatedBoolean {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedBoolean {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedBoolean {
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
impl std::ops::Deref for SVGAnimatedBoolean {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimatedBoolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedBoolean> for emlite::Val {
    fn from(s: SVGAnimatedBoolean) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedBoolean {
    pub fn base_val(&self) -> bool {
        self.inner.get("baseVal").as_::<bool>()
    }

    pub fn set_base_val(&mut self, value: bool) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedBoolean {
    pub fn anim_val(&self) -> bool {
        self.inner.get("animVal").as_::<bool>()
    }
}
