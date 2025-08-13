use super::*;




/// The SVGFETileElement class.
/// [`SVGFETileElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFETileElement {
    inner: SVGElement,
}

impl FromVal for SVGFETileElement {
    fn from_val(v: &Any) -> Self {
        SVGFETileElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFETileElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFETileElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFETileElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFETileElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFETileElement> for Any {
    fn from(s: SVGFETileElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFETileElement> for Any {
    fn from(s: &SVGFETileElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFETileElement);


impl SVGFETileElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFETileElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFETileElement {
    /// Getter of the `x` attribute.
    /// [`SVGFETileElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETileElement {
    /// Getter of the `y` attribute.
    /// [`SVGFETileElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETileElement {
    /// Getter of the `width` attribute.
    /// [`SVGFETileElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETileElement {
    /// Getter of the `height` attribute.
    /// [`SVGFETileElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFETileElement {
    /// Getter of the `result` attribute.
    /// [`SVGFETileElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
