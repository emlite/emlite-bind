use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGGradientElement {
    inner: SVGElement,
}
impl FromVal for SVGGradientElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGGradientElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGGradientElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGGradientElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGGradientElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGGradientElement> for emlite::Val {
    fn from(s: SVGGradientElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGGradientElement);


impl SVGGradientElement {
    pub fn gradient_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("gradientUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGGradientElement {
    pub fn gradient_transform(&self) -> SVGAnimatedTransformList {
        self.inner.get("gradientTransform").as_::<SVGAnimatedTransformList>()
    }

}
impl SVGGradientElement {
    pub fn spread_method(&self) -> SVGAnimatedEnumeration {
        self.inner.get("spreadMethod").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGGradientElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }

}
