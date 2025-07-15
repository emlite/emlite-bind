use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGClipPathElement {
    inner: SVGElement,
}
impl FromVal for SVGClipPathElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGClipPathElement {
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
impl core::ops::Deref for SVGClipPathElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGClipPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGClipPathElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGClipPathElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGClipPathElement> for emlite::Val {
    fn from(s: SVGClipPathElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGClipPathElement);

impl SVGClipPathElement {
    pub fn clip_path_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("clipPathUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGClipPathElement {
    pub fn transform(&self) -> SVGAnimatedTransformList {
        self.inner
            .get("transform")
            .as_::<SVGAnimatedTransformList>()
    }
}
