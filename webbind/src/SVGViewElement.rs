use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGViewElement {
    inner: SVGElement,
}
impl FromVal for SVGViewElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGViewElement {
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
impl core::ops::Deref for SVGViewElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGViewElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGViewElement> for emlite::Val {
    fn from(s: SVGViewElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGViewElement {
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGViewElement {
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
