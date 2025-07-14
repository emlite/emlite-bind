use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGPatternElement {
    inner: SVGElement,
}
impl FromVal for SVGPatternElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPatternElement {
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
impl From<SVGPatternElement> for emlite::Val {
    fn from(s: SVGPatternElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGPatternElement {
    pub fn pattern_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("patternUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGPatternElement {
    pub fn pattern_content_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("patternContentUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGPatternElement {
    pub fn pattern_transform(&self) -> SVGAnimatedTransformList {
        self.inner
            .get("patternTransform")
            .as_::<SVGAnimatedTransformList>()
    }
}
impl SVGPatternElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGPatternElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGPatternElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGPatternElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGPatternElement {
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGPatternElement {
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
impl SVGPatternElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
