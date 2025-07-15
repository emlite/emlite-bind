use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedNumber {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedNumber {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedNumber { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAnimatedNumber {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAnimatedNumber {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAnimatedNumber {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGAnimatedNumber> for emlite::Val {
    fn from(s: SVGAnimatedNumber) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedNumber);


impl SVGAnimatedNumber {
    pub fn base_val(&self) -> f32 {
        self.inner.get("baseVal").as_::<f32>()
    }

    pub fn set_base_val(&mut self, value: f32) {
        self.inner.set("baseVal", value);
    }

}
impl SVGAnimatedNumber {
    pub fn anim_val(&self) -> f32 {
        self.inner.get("animVal").as_::<f32>()
    }

}
