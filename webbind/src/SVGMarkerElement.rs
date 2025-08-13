use super::*;




/// The SVGMarkerElement class.
/// [`SVGMarkerElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGMarkerElement {
    inner: SVGElement,
}

impl FromVal for SVGMarkerElement {
    fn from_val(v: &Any) -> Self {
        SVGMarkerElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGMarkerElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGMarkerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGMarkerElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGMarkerElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGMarkerElement> for Any {
    fn from(s: SVGMarkerElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGMarkerElement> for Any {
    fn from(s: &SVGMarkerElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGMarkerElement);


impl SVGMarkerElement {
    /// Getter of the `refX` attribute.
    /// [`SVGMarkerElement.refX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)
    pub fn ref_x(&self) -> SVGAnimatedLength {
        self.inner.get("refX").as_::<SVGAnimatedLength>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `refY` attribute.
    /// [`SVGMarkerElement.refY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)
    pub fn ref_y(&self) -> SVGAnimatedLength {
        self.inner.get("refY").as_::<SVGAnimatedLength>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `markerUnits` attribute.
    /// [`SVGMarkerElement.markerUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)
    pub fn marker_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("markerUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `markerWidth` attribute.
    /// [`SVGMarkerElement.markerWidth`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)
    pub fn marker_width(&self) -> SVGAnimatedLength {
        self.inner.get("markerWidth").as_::<SVGAnimatedLength>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `markerHeight` attribute.
    /// [`SVGMarkerElement.markerHeight`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)
    pub fn marker_height(&self) -> SVGAnimatedLength {
        self.inner.get("markerHeight").as_::<SVGAnimatedLength>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `orientType` attribute.
    /// [`SVGMarkerElement.orientType`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)
    pub fn orient_type(&self) -> SVGAnimatedEnumeration {
        self.inner.get("orientType").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `orientAngle` attribute.
    /// [`SVGMarkerElement.orientAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)
    pub fn orient_angle(&self) -> SVGAnimatedAngle {
        self.inner.get("orientAngle").as_::<SVGAnimatedAngle>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `orient` attribute.
    /// [`SVGMarkerElement.orient`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orient)
    pub fn orient(&self) -> JsString {
        self.inner.get("orient").as_::<JsString>()
    }

    /// Setter of the `orient` attribute.
    /// [`SVGMarkerElement.orient`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orient)
    pub fn set_orient(&mut self, value: &JsString) {
        self.inner.set("orient", value);
    }
}
impl SVGMarkerElement {
    /// The setOrientToAuto method.
    /// [`SVGMarkerElement.setOrientToAuto`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)
    pub fn set_orient_to_auto(&self, ) -> Undefined {
        self.inner.call("setOrientToAuto", &[]).as_::<Undefined>()
    }
}
impl SVGMarkerElement {
    /// The setOrientToAngle method.
    /// [`SVGMarkerElement.setOrientToAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)
    pub fn set_orient_to_angle(&self, angle: &SVGAngle) -> Undefined {
        self.inner.call("setOrientToAngle", &[angle.into(), ]).as_::<Undefined>()
    }
}
impl SVGMarkerElement {
    /// Getter of the `viewBox` attribute.
    /// [`SVGMarkerElement.viewBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }

}
impl SVGMarkerElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGMarkerElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner.get("preserveAspectRatio").as_::<SVGAnimatedPreserveAspectRatio>()
    }

}
