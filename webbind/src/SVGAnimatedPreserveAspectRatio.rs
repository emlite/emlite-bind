use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedPreserveAspectRatio {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedPreserveAspectRatio {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedPreserveAspectRatio { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAnimatedPreserveAspectRatio {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedPreserveAspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAnimatedPreserveAspectRatio {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAnimatedPreserveAspectRatio {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGAnimatedPreserveAspectRatio> for emlite::Val {
    fn from(s: SVGAnimatedPreserveAspectRatio) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedPreserveAspectRatio);


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
