use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGPathDataSettings {
    inner: emlite::Val,
}
impl FromVal for SVGPathDataSettings {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPathDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGPathDataSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGPathDataSettings> for emlite::Val {
    fn from(s: SVGPathDataSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGPathDataSettings {
    pub fn normalize(&self) -> bool {
        self.inner.get("normalize").as_::<bool>()
    }

    pub fn set_normalize(&mut self, value: bool) {
        self.inner.set("normalize", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGPathElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGPathElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPathElement {
            inner: SVGGeometryElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGPathElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGPathElement> for emlite::Val {
    fn from(s: SVGPathElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGPathElement {
    pub fn path_length(&self) -> SVGAnimatedNumber {
        self.inner.get("pathLength").as_::<SVGAnimatedNumber>()
    }
}
impl SVGPathElement {
    pub fn get_total_length(&self) -> f32 {
        self.inner.call("getTotalLength", &[]).as_::<f32>()
    }
}
impl SVGPathElement {
    pub fn get_point_at_length(&self, distance: f32) -> DOMPoint {
        self.inner
            .call("getPointAtLength", &[distance.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPathElement {
    pub fn get_path_segment_at_length(&self, distance: f32) -> SVGPathSegment {
        self.inner
            .call("getPathSegmentAtLength", &[distance.into()])
            .as_::<SVGPathSegment>()
    }
}
impl SVGPathElement {
    pub fn get_path_data0(&self) -> jsbind::Sequence<SVGPathSegment> {
        self.inner
            .call("getPathData", &[])
            .as_::<jsbind::Sequence<SVGPathSegment>>()
    }

    pub fn get_path_data1(
        &self,
        settings: SVGPathDataSettings,
    ) -> jsbind::Sequence<SVGPathSegment> {
        self.inner
            .call("getPathData", &[settings.into()])
            .as_::<jsbind::Sequence<SVGPathSegment>>()
    }
}
impl SVGPathElement {
    pub fn set_path_data(&self, path_data: jsbind::Sequence<SVGPathSegment>) -> jsbind::Undefined {
        self.inner
            .call("setPathData", &[path_data.into()])
            .as_::<jsbind::Undefined>()
    }
}
