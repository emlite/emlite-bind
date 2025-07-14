use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEGaussianBlurElement {
    inner: SVGElement,
}
impl FromVal for SVGFEGaussianBlurElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEGaussianBlurElement {
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
impl core::ops::Deref for SVGFEGaussianBlurElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEGaussianBlurElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEGaussianBlurElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEGaussianBlurElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFEGaussianBlurElement> for emlite::Val {
    fn from(s: SVGFEGaussianBlurElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEGaussianBlurElement);

impl SVGFEGaussianBlurElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn std_deviation_x(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn std_deviation_y(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn edge_mode(&self) -> SVGAnimatedEnumeration {
        self.inner.get("edgeMode").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn set_std_deviation(
        &self,
        std_deviation_x: f32,
        std_deviation_y: f32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setStdDeviation",
                &[std_deviation_x.into(), std_deviation_y.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
