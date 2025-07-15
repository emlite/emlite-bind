use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFETurbulenceElement {
    inner: SVGElement,
}
impl FromVal for SVGFETurbulenceElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFETurbulenceElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGFETurbulenceElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFETurbulenceElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFETurbulenceElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFETurbulenceElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGFETurbulenceElement> for emlite::Val {
    fn from(s: SVGFETurbulenceElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFETurbulenceElement);


impl SVGFETurbulenceElement {
    pub fn base_frequency_x(&self) -> SVGAnimatedNumber {
        self.inner.get("baseFrequencyX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    pub fn base_frequency_y(&self) -> SVGAnimatedNumber {
        self.inner.get("baseFrequencyY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    pub fn num_octaves(&self) -> SVGAnimatedInteger {
        self.inner.get("numOctaves").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFETurbulenceElement {
    pub fn seed(&self) -> SVGAnimatedNumber {
        self.inner.get("seed").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    pub fn stitch_tiles(&self) -> SVGAnimatedEnumeration {
        self.inner.get("stitchTiles").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFETurbulenceElement {
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFETurbulenceElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
