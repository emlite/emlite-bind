use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDisplacementMapElement {
    inner: SVGElement,
}
impl FromVal for SVGFEDisplacementMapElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEDisplacementMapElement {
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
impl core::ops::Deref for SVGFEDisplacementMapElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEDisplacementMapElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEDisplacementMapElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEDisplacementMapElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFEDisplacementMapElement> for emlite::Val {
    fn from(s: SVGFEDisplacementMapElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEDisplacementMapElement);

impl SVGFEDisplacementMapElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn in2(&self) -> SVGAnimatedString {
        self.inner.get("in2").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn scale(&self) -> SVGAnimatedNumber {
        self.inner.get("scale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn x_channel_selector(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("xChannelSelector")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn y_channel_selector(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("yChannelSelector")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
