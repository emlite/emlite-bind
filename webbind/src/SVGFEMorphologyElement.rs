use super::*;




/// The SVGFEMorphologyElement class.
/// [`SVGFEMorphologyElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEMorphologyElement {
    inner: SVGElement,
}

impl FromVal for SVGFEMorphologyElement {
    fn from_val(v: &Any) -> Self {
        SVGFEMorphologyElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFEMorphologyElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEMorphologyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEMorphologyElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEMorphologyElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFEMorphologyElement> for Any {
    fn from(s: SVGFEMorphologyElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEMorphologyElement> for Any {
    fn from(s: &SVGFEMorphologyElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEMorphologyElement);


impl SVGFEMorphologyElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEMorphologyElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `operator` attribute.
    /// [`SVGFEMorphologyElement.operator`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/operator)
    pub fn operator(&self) -> SVGAnimatedEnumeration {
        self.inner.get("operator").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `radiusX` attribute.
    /// [`SVGFEMorphologyElement.radiusX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusX)
    pub fn radius_x(&self) -> SVGAnimatedNumber {
        self.inner.get("radiusX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `radiusY` attribute.
    /// [`SVGFEMorphologyElement.radiusY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusY)
    pub fn radius_y(&self) -> SVGAnimatedNumber {
        self.inner.get("radiusY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEMorphologyElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEMorphologyElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEMorphologyElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEMorphologyElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEMorphologyElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEMorphologyElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
