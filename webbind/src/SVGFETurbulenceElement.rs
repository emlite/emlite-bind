use super::*;




/// The SVGFETurbulenceElement class.
/// [`SVGFETurbulenceElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFETurbulenceElement {
    inner: SVGElement,
}

impl FromVal for SVGFETurbulenceElement {
    fn from_val(v: &Any) -> Self {
        SVGFETurbulenceElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SVGFETurbulenceElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFETurbulenceElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFETurbulenceElement> for Any {
    fn from(s: SVGFETurbulenceElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFETurbulenceElement> for Any {
    fn from(s: &SVGFETurbulenceElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFETurbulenceElement);


impl SVGFETurbulenceElement {
    /// Getter of the `baseFrequencyX` attribute.
    /// [`SVGFETurbulenceElement.baseFrequencyX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyX)
    pub fn base_frequency_x(&self) -> SVGAnimatedNumber {
        self.inner.get("baseFrequencyX").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `baseFrequencyY` attribute.
    /// [`SVGFETurbulenceElement.baseFrequencyY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyY)
    pub fn base_frequency_y(&self) -> SVGAnimatedNumber {
        self.inner.get("baseFrequencyY").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `numOctaves` attribute.
    /// [`SVGFETurbulenceElement.numOctaves`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/numOctaves)
    pub fn num_octaves(&self) -> SVGAnimatedInteger {
        self.inner.get("numOctaves").as_::<SVGAnimatedInteger>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `seed` attribute.
    /// [`SVGFETurbulenceElement.seed`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/seed)
    pub fn seed(&self) -> SVGAnimatedNumber {
        self.inner.get("seed").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `stitchTiles` attribute.
    /// [`SVGFETurbulenceElement.stitchTiles`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/stitchTiles)
    pub fn stitch_tiles(&self) -> SVGAnimatedEnumeration {
        self.inner.get("stitchTiles").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `type` attribute.
    /// [`SVGFETurbulenceElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/type)
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `x` attribute.
    /// [`SVGFETurbulenceElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `y` attribute.
    /// [`SVGFETurbulenceElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `width` attribute.
    /// [`SVGFETurbulenceElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `height` attribute.
    /// [`SVGFETurbulenceElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETurbulenceElement {
    /// Getter of the `result` attribute.
    /// [`SVGFETurbulenceElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
