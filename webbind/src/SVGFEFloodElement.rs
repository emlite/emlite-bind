use super::*;




/// The SVGFEFloodElement class.
/// [`SVGFEFloodElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEFloodElement {
    inner: SVGElement,
}

impl FromVal for SVGFEFloodElement {
    fn from_val(v: &Any) -> Self {
        SVGFEFloodElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFEFloodElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEFloodElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEFloodElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEFloodElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFEFloodElement> for Any {
    fn from(s: SVGFEFloodElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEFloodElement> for Any {
    fn from(s: &SVGFEFloodElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEFloodElement);


impl SVGFEFloodElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEFloodElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEFloodElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEFloodElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEFloodElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEFloodElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEFloodElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEFloodElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEFloodElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEFloodElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
