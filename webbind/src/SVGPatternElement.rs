use super::*;




/// The SVGPatternElement class.
/// [`SVGPatternElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPatternElement {
    inner: SVGElement,
}

impl FromVal for SVGPatternElement {
    fn from_val(v: &Any) -> Self {
        SVGPatternElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGPatternElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGPatternElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGPatternElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGPatternElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGPatternElement> for Any {
    fn from(s: SVGPatternElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGPatternElement> for Any {
    fn from(s: &SVGPatternElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGPatternElement);


impl SVGPatternElement {
    /// Getter of the `patternUnits` attribute.
    /// [`SVGPatternElement.patternUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternUnits)
    pub fn pattern_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("patternUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGPatternElement {
    /// Getter of the `patternContentUnits` attribute.
    /// [`SVGPatternElement.patternContentUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternContentUnits)
    pub fn pattern_content_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("patternContentUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGPatternElement {
    /// Getter of the `patternTransform` attribute.
    /// [`SVGPatternElement.patternTransform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternTransform)
    pub fn pattern_transform(&self) -> SVGAnimatedTransformList {
        self.inner.get("patternTransform").as_::<SVGAnimatedTransformList>()
    }

}
impl SVGPatternElement {
    /// Getter of the `x` attribute.
    /// [`SVGPatternElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }

}
impl SVGPatternElement {
    /// Getter of the `y` attribute.
    /// [`SVGPatternElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }

}
impl SVGPatternElement {
    /// Getter of the `width` attribute.
    /// [`SVGPatternElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }

}
impl SVGPatternElement {
    /// Getter of the `height` attribute.
    /// [`SVGPatternElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }

}
impl SVGPatternElement {
    /// Getter of the `viewBox` attribute.
    /// [`SVGPatternElement.viewBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/viewBox)
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }

}
impl SVGPatternElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGPatternElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner.get("preserveAspectRatio").as_::<SVGAnimatedPreserveAspectRatio>()
    }

}
impl SVGPatternElement {
    /// Getter of the `href` attribute.
    /// [`SVGPatternElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }

}
