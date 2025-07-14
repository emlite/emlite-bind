use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGMarkerElement {
    inner: SVGElement,
}
impl FromVal for SVGMarkerElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGMarkerElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<SVGMarkerElement> for emlite::Val {
    fn from(s: SVGMarkerElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGMarkerElement {
    pub fn ref_x(&self) -> SVGAnimatedLength {
        self.inner.get("refX").as_::<SVGAnimatedLength>()
    }
}
impl SVGMarkerElement {
    pub fn ref_y(&self) -> SVGAnimatedLength {
        self.inner.get("refY").as_::<SVGAnimatedLength>()
    }
}
impl SVGMarkerElement {
    pub fn marker_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("markerUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGMarkerElement {
    pub fn marker_width(&self) -> SVGAnimatedLength {
        self.inner.get("markerWidth").as_::<SVGAnimatedLength>()
    }
}
impl SVGMarkerElement {
    pub fn marker_height(&self) -> SVGAnimatedLength {
        self.inner.get("markerHeight").as_::<SVGAnimatedLength>()
    }
}
impl SVGMarkerElement {
    pub fn orient_type(&self) -> SVGAnimatedEnumeration {
        self.inner.get("orientType").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGMarkerElement {
    pub fn orient_angle(&self) -> SVGAnimatedAngle {
        self.inner.get("orientAngle").as_::<SVGAnimatedAngle>()
    }
}
impl SVGMarkerElement {
    pub fn orient(&self) -> jsbind::DOMString {
        self.inner.get("orient").as_::<jsbind::DOMString>()
    }

    pub fn set_orient(&mut self, value: jsbind::DOMString) {
        self.inner.set("orient", value);
    }
}
impl SVGMarkerElement {
    pub fn set_orient_to_auto(&self) -> jsbind::Undefined {
        self.inner
            .call("setOrientToAuto", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGMarkerElement {
    pub fn set_orient_to_angle(&self, angle: SVGAngle) -> jsbind::Undefined {
        self.inner
            .call("setOrientToAngle", &[angle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGMarkerElement {
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGMarkerElement {
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
