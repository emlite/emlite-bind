use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPolylineElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGPolylineElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPolylineElement {
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
impl core::ops::Deref for SVGPolylineElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPolylineElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGPolylineElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGPolylineElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGPolylineElement> for emlite::Val {
    fn from(s: SVGPolylineElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGPolylineElement> for emlite::Val {
    fn from(s: &SVGPolylineElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPolylineElement);

impl SVGPolylineElement {
    pub fn points(&self) -> SVGPointList {
        self.inner.get("points").as_::<SVGPointList>()
    }
}
impl SVGPolylineElement {
    pub fn animated_points(&self) -> SVGPointList {
        self.inner.get("animatedPoints").as_::<SVGPointList>()
    }
}
