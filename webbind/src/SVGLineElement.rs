use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLineElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGLineElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGLineElement {
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
impl core::ops::Deref for SVGLineElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGLineElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGLineElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGLineElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGLineElement> for emlite::Val {
    fn from(s: SVGLineElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGLineElement> for emlite::Val {
    fn from(s: &SVGLineElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGLineElement);

impl SVGLineElement {
    pub fn x1(&self) -> SVGAnimatedLength {
        self.inner.get("x1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn y1(&self) -> SVGAnimatedLength {
        self.inner.get("y1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn x2(&self) -> SVGAnimatedLength {
        self.inner.get("x2").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn y2(&self) -> SVGAnimatedLength {
        self.inner.get("y2").as_::<SVGAnimatedLength>()
    }
}
