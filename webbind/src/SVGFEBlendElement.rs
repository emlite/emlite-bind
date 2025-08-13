use super::*;




/// The SVGFEBlendElement class.
/// [`SVGFEBlendElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEBlendElement {
    inner: SVGElement,
}

impl FromVal for SVGFEBlendElement {
    fn from_val(v: &Any) -> Self {
        SVGFEBlendElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFEBlendElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEBlendElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEBlendElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEBlendElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFEBlendElement> for Any {
    fn from(s: SVGFEBlendElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEBlendElement> for Any {
    fn from(s: &SVGFEBlendElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEBlendElement);


impl SVGFEBlendElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEBlendElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `in2` attribute.
    /// [`SVGFEBlendElement.in2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in2)
    pub fn in2(&self) -> SVGAnimatedString {
        self.inner.get("in2").as_::<SVGAnimatedString>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `mode` attribute.
    /// [`SVGFEBlendElement.mode`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/mode)
    pub fn mode(&self) -> SVGAnimatedEnumeration {
        self.inner.get("mode").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEBlendElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEBlendElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEBlendElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEBlendElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGFEBlendElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEBlendElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }

}
