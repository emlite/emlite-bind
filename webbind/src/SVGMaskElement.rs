use super::*;




/// The SVGMaskElement class.
/// [`SVGMaskElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGMaskElement {
    inner: SVGElement,
}

impl FromVal for SVGMaskElement {
    fn from_val(v: &Any) -> Self {
        SVGMaskElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGMaskElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGMaskElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGMaskElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGMaskElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGMaskElement> for Any {
    fn from(s: SVGMaskElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGMaskElement> for Any {
    fn from(s: &SVGMaskElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGMaskElement);


impl SVGMaskElement {
    /// Getter of the `maskUnits` attribute.
    /// [`SVGMaskElement.maskUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskUnits)
    pub fn mask_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("maskUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGMaskElement {
    /// Getter of the `maskContentUnits` attribute.
    /// [`SVGMaskElement.maskContentUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskContentUnits)
    pub fn mask_content_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("maskContentUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGMaskElement {
    /// Getter of the `x` attribute.
    /// [`SVGMaskElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGMaskElement {
    /// Getter of the `y` attribute.
    /// [`SVGMaskElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGMaskElement {
    /// Getter of the `width` attribute.
    /// [`SVGMaskElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGMaskElement {
    /// Getter of the `height` attribute.
    /// [`SVGMaskElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
