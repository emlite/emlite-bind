use super::*;




/// The SVGForeignObjectElement class.
/// [`SVGForeignObjectElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGForeignObjectElement {
    inner: SVGGraphicsElement,
}

impl FromVal for SVGForeignObjectElement {
    fn from_val(v: &Any) -> Self {
        SVGForeignObjectElement { inner: SVGGraphicsElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGForeignObjectElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGForeignObjectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGForeignObjectElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGForeignObjectElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGForeignObjectElement> for Any {
    fn from(s: SVGForeignObjectElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGForeignObjectElement> for Any {
    fn from(s: &SVGForeignObjectElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGForeignObjectElement);


impl SVGForeignObjectElement {
    /// Getter of the `x` attribute.
    /// [`SVGForeignObjectElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGForeignObjectElement {
    /// Getter of the `y` attribute.
    /// [`SVGForeignObjectElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGForeignObjectElement {
    /// Getter of the `width` attribute.
    /// [`SVGForeignObjectElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGForeignObjectElement {
    /// Getter of the `height` attribute.
    /// [`SVGForeignObjectElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
