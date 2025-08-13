use super::*;




/// The SVGFEConvolveMatrixElement class.
/// [`SVGFEConvolveMatrixElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEConvolveMatrixElement {
    inner: SVGElement,
}

impl FromVal for SVGFEConvolveMatrixElement {
    fn from_val(v: &Any) -> Self {
        SVGFEConvolveMatrixElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SVGFEConvolveMatrixElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEConvolveMatrixElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFEConvolveMatrixElement> for Any {
    fn from(s: SVGFEConvolveMatrixElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEConvolveMatrixElement> for Any {
    fn from(s: &SVGFEConvolveMatrixElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEConvolveMatrixElement);


impl SVGFEConvolveMatrixElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEConvolveMatrixElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `orderX` attribute.
    /// [`SVGFEConvolveMatrixElement.orderX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderX)
    pub fn order_x(&self) -> SVGAnimatedInteger {
        self.inner.get("orderX").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `orderY` attribute.
    /// [`SVGFEConvolveMatrixElement.orderY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderY)
    pub fn order_y(&self) -> SVGAnimatedInteger {
        self.inner.get("orderY").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `kernelMatrix` attribute.
    /// [`SVGFEConvolveMatrixElement.kernelMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelMatrix)
    pub fn kernel_matrix(&self) -> SVGAnimatedNumberList {
        self.inner.get("kernelMatrix").as_::<SVGAnimatedNumberList>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `divisor` attribute.
    /// [`SVGFEConvolveMatrixElement.divisor`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/divisor)
    pub fn divisor(&self) -> SVGAnimatedNumber {
        self.inner.get("divisor").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `bias` attribute.
    /// [`SVGFEConvolveMatrixElement.bias`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/bias)
    pub fn bias(&self) -> SVGAnimatedNumber {
        self.inner.get("bias").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `targetX` attribute.
    /// [`SVGFEConvolveMatrixElement.targetX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetX)
    pub fn target_x(&self) -> SVGAnimatedInteger {
        self.inner.get("targetX").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `targetY` attribute.
    /// [`SVGFEConvolveMatrixElement.targetY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetY)
    pub fn target_y(&self) -> SVGAnimatedInteger {
        self.inner.get("targetY").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `edgeMode` attribute.
    /// [`SVGFEConvolveMatrixElement.edgeMode`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/edgeMode)
    pub fn edge_mode(&self) -> SVGAnimatedEnumeration {
        self.inner.get("edgeMode").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `kernelUnitLengthX` attribute.
    /// [`SVGFEConvolveMatrixElement.kernelUnitLengthX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthX)
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner.get("kernelUnitLengthX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `kernelUnitLengthY` attribute.
    /// [`SVGFEConvolveMatrixElement.kernelUnitLengthY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthY)
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner.get("kernelUnitLengthY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `preserveAlpha` attribute.
    /// [`SVGFEConvolveMatrixElement.preserveAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/preserveAlpha)
    pub fn preserve_alpha(&self) -> SVGAnimatedBoolean {
        self.inner.get("preserveAlpha").as_::<SVGAnimatedBoolean>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEConvolveMatrixElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEConvolveMatrixElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEConvolveMatrixElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEConvolveMatrixElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEConvolveMatrixElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEConvolveMatrixElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
