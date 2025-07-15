use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDropShadowElement {
    inner: SVGElement,
}
impl FromVal for SVGFEDropShadowElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEDropShadowElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGFEDropShadowElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEDropShadowElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEDropShadowElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEDropShadowElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGFEDropShadowElement> for emlite::Val {
    fn from(s: SVGFEDropShadowElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEDropShadowElement);


impl SVGFEDropShadowElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFEDropShadowElement {
    pub fn dx(&self) -> SVGAnimatedNumber {
        self.inner.get("dx").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEDropShadowElement {
    pub fn dy(&self) -> SVGAnimatedNumber {
        self.inner.get("dy").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEDropShadowElement {
    pub fn std_deviation_x(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEDropShadowElement {
    pub fn std_deviation_y(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEDropShadowElement {
    pub fn set_std_deviation(&self, std_deviation_x: f32, std_deviation_y: f32) -> Undefined {
        self.inner.call("setStdDeviation", &[std_deviation_x.into(), std_deviation_y.into(), ]).as_::<Undefined>()
    }

}
impl SVGFEDropShadowElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEDropShadowElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEDropShadowElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEDropShadowElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEDropShadowElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
