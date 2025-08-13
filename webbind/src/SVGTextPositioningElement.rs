use super::*;




/// The SVGTextPositioningElement class.
/// [`SVGTextPositioningElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTextPositioningElement {
    inner: SVGTextContentElement,
}

impl FromVal for SVGTextPositioningElement {
    fn from_val(v: &Any) -> Self {
        SVGTextPositioningElement { inner: SVGTextContentElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGTextPositioningElement {
    type Target = SVGTextContentElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGTextPositioningElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGTextPositioningElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGTextPositioningElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGTextPositioningElement> for Any {
    fn from(s: SVGTextPositioningElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGTextPositioningElement> for Any {
    fn from(s: &SVGTextPositioningElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGTextPositioningElement);


impl SVGTextPositioningElement {
    /// Getter of the `x` attribute.
    /// [`SVGTextPositioningElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/x)
    pub fn x(&self) -> SVGAnimatedLengthList {
        self.inner.get("x").as_::<SVGAnimatedLengthList>()
    }

}
impl SVGTextPositioningElement {
    /// Getter of the `y` attribute.
    /// [`SVGTextPositioningElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/y)
    pub fn y(&self) -> SVGAnimatedLengthList {
        self.inner.get("y").as_::<SVGAnimatedLengthList>()
    }

}
impl SVGTextPositioningElement {
    /// Getter of the `dx` attribute.
    /// [`SVGTextPositioningElement.dx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dx)
    pub fn dx(&self) -> SVGAnimatedLengthList {
        self.inner.get("dx").as_::<SVGAnimatedLengthList>()
    }

}
impl SVGTextPositioningElement {
    /// Getter of the `dy` attribute.
    /// [`SVGTextPositioningElement.dy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dy)
    pub fn dy(&self) -> SVGAnimatedLengthList {
        self.inner.get("dy").as_::<SVGAnimatedLengthList>()
    }

}
impl SVGTextPositioningElement {
    /// Getter of the `rotate` attribute.
    /// [`SVGTextPositioningElement.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/rotate)
    pub fn rotate(&self) -> SVGAnimatedNumberList {
        self.inner.get("rotate").as_::<SVGAnimatedNumberList>()
    }

}
