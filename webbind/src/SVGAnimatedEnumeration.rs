use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimatedEnumeration {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedEnumeration {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedEnumeration {
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
impl std::ops::Deref for SVGAnimatedEnumeration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimatedEnumeration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedEnumeration> for emlite::Val {
    fn from(s: SVGAnimatedEnumeration) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedEnumeration {
    pub fn base_val(&self) -> u16 {
        self.inner.get("baseVal").as_::<u16>()
    }

    pub fn set_base_val(&mut self, value: u16) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedEnumeration {
    pub fn anim_val(&self) -> u16 {
        self.inner.get("animVal").as_::<u16>()
    }
}
