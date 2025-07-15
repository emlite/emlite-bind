use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEConvolveMatrixElement {
    inner: SVGElement,
}
impl FromVal for SVGFEConvolveMatrixElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEConvolveMatrixElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGFEConvolveMatrixElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEConvolveMatrixElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEConvolveMatrixElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEConvolveMatrixElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGFEConvolveMatrixElement> for emlite::Val {
    fn from(s: SVGFEConvolveMatrixElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEConvolveMatrixElement);


impl SVGFEConvolveMatrixElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn order_x(&self) -> SVGAnimatedInteger {
        self.inner.get("orderX").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn order_y(&self) -> SVGAnimatedInteger {
        self.inner.get("orderY").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn kernel_matrix(&self) -> SVGAnimatedNumberList {
        self.inner.get("kernelMatrix").as_::<SVGAnimatedNumberList>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn divisor(&self) -> SVGAnimatedNumber {
        self.inner.get("divisor").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn bias(&self) -> SVGAnimatedNumber {
        self.inner.get("bias").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn target_x(&self) -> SVGAnimatedInteger {
        self.inner.get("targetX").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn target_y(&self) -> SVGAnimatedInteger {
        self.inner.get("targetY").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn edge_mode(&self) -> SVGAnimatedEnumeration {
        self.inner.get("edgeMode").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner.get("kernelUnitLengthX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner.get("kernelUnitLengthY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn preserve_alpha(&self) -> SVGAnimatedBoolean {
        self.inner.get("preserveAlpha").as_::<SVGAnimatedBoolean>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
