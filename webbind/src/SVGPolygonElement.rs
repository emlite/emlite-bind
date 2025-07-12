use super::*;

#[derive(Clone, Debug)]
pub struct SVGPolygonElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGPolygonElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPolygonElement {
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
impl std::ops::Deref for SVGPolygonElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGPolygonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGPolygonElement> for emlite::Val {
    fn from(s: SVGPolygonElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGPolygonElement {
    pub fn points(&self) -> SVGPointList {
        self.inner.get("points").as_::<SVGPointList>()
    }
}
impl SVGPolygonElement {
    pub fn animated_points(&self) -> SVGPointList {
        self.inner.get("animatedPoints").as_::<SVGPointList>()
    }
}
