use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGAnimatedInteger {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedInteger {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedInteger {
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
impl core::ops::Deref for SVGAnimatedInteger {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedInteger> for emlite::Val {
    fn from(s: SVGAnimatedInteger) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedInteger {
    pub fn base_val(&self) -> i32 {
        self.inner.get("baseVal").as_::<i32>()
    }

    pub fn set_base_val(&mut self, value: i32) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedInteger {
    pub fn anim_val(&self) -> i32 {
        self.inner.get("animVal").as_::<i32>()
    }
}
