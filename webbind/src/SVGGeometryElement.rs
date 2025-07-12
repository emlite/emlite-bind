use super::*;

#[derive(Clone, Debug)]
pub struct DOMPointInit {
    inner: emlite::Val,
}
impl FromVal for DOMPointInit {
    fn from_val(v: &emlite::Val) -> Self {
        DOMPointInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DOMPointInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DOMPointInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMPointInit> for emlite::Val {
    fn from(s: DOMPointInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMPointInit {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMPointInit {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMPointInit {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl DOMPointInit {
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}
#[derive(Clone, Debug)]
pub struct SVGGeometryElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGGeometryElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGGeometryElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGGeometryElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGGeometryElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGGeometryElement> for emlite::Val {
    fn from(s: SVGGeometryElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGGeometryElement {
    pub fn path_length(&self) -> SVGAnimatedNumber {
        self.inner.get("pathLength").as_::<SVGAnimatedNumber>()
    }
}
impl SVGGeometryElement {
    pub fn is_point_in_fill0(&self) -> bool {
        self.inner.call("isPointInFill", &[]).as_::<bool>()
    }

    pub fn is_point_in_fill1(&self, point: DOMPointInit) -> bool {
        self.inner
            .call("isPointInFill", &[point.into()])
            .as_::<bool>()
    }
}
impl SVGGeometryElement {
    pub fn is_point_in_stroke0(&self) -> bool {
        self.inner.call("isPointInStroke", &[]).as_::<bool>()
    }

    pub fn is_point_in_stroke1(&self, point: DOMPointInit) -> bool {
        self.inner
            .call("isPointInStroke", &[point.into()])
            .as_::<bool>()
    }
}
impl SVGGeometryElement {
    pub fn get_total_length(&self) -> f32 {
        self.inner.call("getTotalLength", &[]).as_::<f32>()
    }
}
impl SVGGeometryElement {
    pub fn get_point_at_length(&self, distance: f32) -> DOMPoint {
        self.inner
            .call("getPointAtLength", &[distance.into()])
            .as_::<DOMPoint>()
    }
}
