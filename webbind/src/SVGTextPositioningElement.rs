use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGTextPositioningElement {
    inner: SVGTextContentElement,
}
impl FromVal for SVGTextPositioningElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTextPositioningElement {
            inner: SVGTextContentElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGTextPositioningElement {
    type Target = SVGTextContentElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGTextPositioningElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGTextPositioningElement> for emlite::Val {
    fn from(s: SVGTextPositioningElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGTextPositioningElement {
    pub fn x(&self) -> SVGAnimatedLengthList {
        self.inner.get("x").as_::<SVGAnimatedLengthList>()
    }
}
impl SVGTextPositioningElement {
    pub fn y(&self) -> SVGAnimatedLengthList {
        self.inner.get("y").as_::<SVGAnimatedLengthList>()
    }
}
impl SVGTextPositioningElement {
    pub fn dx(&self) -> SVGAnimatedLengthList {
        self.inner.get("dx").as_::<SVGAnimatedLengthList>()
    }
}
impl SVGTextPositioningElement {
    pub fn dy(&self) -> SVGAnimatedLengthList {
        self.inner.get("dy").as_::<SVGAnimatedLengthList>()
    }
}
impl SVGTextPositioningElement {
    pub fn rotate(&self) -> SVGAnimatedNumberList {
        self.inner.get("rotate").as_::<SVGAnimatedNumberList>()
    }
}
